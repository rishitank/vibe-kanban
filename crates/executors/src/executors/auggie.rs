use std::{path::PathBuf, process::Stdio, sync::Arc};

use async_trait::async_trait;
use command_group::{AsyncCommandGroup, AsyncGroupChild};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use ts_rs::TS;
use utils::{msg_store::MsgStore, shell::get_shell_command};

use crate::{
    command::CommandBuilder,
    executors::{ExecutorError, StandardCodingAgentExecutor},
    logs::{
        NormalizedEntry, NormalizedEntryType, plain_text_processor::PlainTextLogProcessor,
        stderr_processor::normalize_stderr_logs, utils::EntryIndexProvider,
    },
};

fn shell_quote_single(s: &str) -> String {
    let escaped = s.replace('"', "\\\"");
    format!("\"{}\"", escaped)
}


/// Executor for running Auggie CLI and normalizing its output
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
pub struct Auggie {
    pub command: CommandBuilder,
}

#[async_trait]
impl StandardCodingAgentExecutor for Auggie {
    async fn spawn(
        &self,
        current_dir: &PathBuf,
        prompt: &str,
    ) -> Result<AsyncGroupChild, ExecutorError> {
        let (shell_cmd, shell_arg) = get_shell_command();
        let base_cmd = self.command.build_initial();
        let quoted_prompt = shell_quote_single(prompt);

        // Build MCP config flags: profile can supply a path; if not, try agent defaults
        let mut mcp_args: Vec<String> = Vec::new();
        if let Some(profile) = crate::profile::ProfileConfigs::get_cached()
            .get_profile("auggie")
            .and_then(|p| p.get_mcp_config_path())
        {
            mcp_args.push(format!("--mcp-config {}", profile.display()));
        }

        let agent_cmd = if mcp_args.is_empty() {
            format!("{} {}", base_cmd, quoted_prompt)
        } else {
            format!("{} {} {}", base_cmd, mcp_args.join(" "), quoted_prompt)
        };

        let mut command = Command::new(shell_cmd);
        command
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(current_dir)
            .arg(shell_arg)
            .arg(&agent_cmd);

        let child = command.group_spawn()?;
        Ok(child)
    }

    async fn spawn_follow_up(
        &self,
        _current_dir: &PathBuf,
        _prompt: &str,
        _session_id: &str,
    ) -> Result<AsyncGroupChild, ExecutorError> {
        Err(ExecutorError::FollowUpNotSupported(
            "Auggie CLI follow-up sessions are not yet supported".to_string(),
        ))
    }

    fn normalize_logs(&self, msg_store: Arc<MsgStore>, _worktree_path: &PathBuf) {
        let entry_index_provider = EntryIndexProvider::new();

        // Standardize stderr as ErrorMessage entries
        normalize_stderr_logs(msg_store.clone(), entry_index_provider.clone());

        // Treat stdout as assistant messages using the plain text processor
        tokio::spawn(async move {
            let mut stdout_stream = msg_store.stdout_chunked_stream();

            let mut processor = PlainTextLogProcessor::builder()
                .normalized_entry_producer(Box::new(|content: String| NormalizedEntry {
                    timestamp: None,
                    entry_type: NormalizedEntryType::AssistantMessage,
                    content,
                    metadata: None,
                }))
                .index_provider(entry_index_provider)
                .build();

            while let Some(Ok(chunk)) = stdout_stream.next().await {
                for patch in processor.process(chunk) {
                    msg_store.push_patch(patch);
                }
            }
        });
    }
}

