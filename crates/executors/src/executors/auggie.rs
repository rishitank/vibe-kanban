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
    profile::ProfileConfig,
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

impl Auggie {
    /// Build the agent command string from base command, profile flags, and a quoted prompt.
    /// Exposed for tests to verify flag ordering and presence.
    pub fn build_agent_cmd(base_cmd: &str, profile: Option<&ProfileConfig>, quoted_prompt: &str) -> String {
        let mut flags: Vec<String> = Vec::new();
        if let Some(profile) = profile {
            for path in profile.get_mcp_config_paths() {
                flags.push(format!("--mcp-config {}", path.display()));
            }
            for f in profile.get_auggie_flags() {
                flags.push(f);
            }
        }
        if flags.is_empty() {
            format!("{} {}", base_cmd, quoted_prompt)
        } else {
            format!("{} {} {}", base_cmd, flags.join(" "), quoted_prompt)
        }
    }

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
        // Fetch profile once; if follow-ups are enabled, log an info to set expectations
        let cached = crate::profile::ProfileConfigs::get_cached();
        let profile = cached.get_profile("auggie");
        if let Some(p) = profile {
            if p.auggie_use_continue_followup() {
                tracing::info!(
                    "Auggie follow-ups enabled: VK will use --continue and ignore session_id to resume most recent session"
                );
            }
        }



        let agent_cmd = Self::build_agent_cmd(
            &base_cmd,
            crate::profile::ProfileConfigs::get_cached().get_profile("auggie"),
            &quoted_prompt,
        );

        // TODO: When Auggie supports attaching to a previous session, this can be implemented:
        // let followup_profile = crate::profile::ProfileConfigs::get_cached().get_profile("auggie");
        // let base_cmd = self.command.build_follow_up(&[]);
        // let quoted = shell_quote_single(prompt);
        // let _cmd = Self::build_agent_cmd(&base_cmd, followup_profile, &quoted);

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
        current_dir: &PathBuf,
        prompt: &str,
        session_id: &str,
    ) -> Result<AsyncGroupChild, ExecutorError> {
        let (shell_cmd, shell_arg) = get_shell_command();
        let base_cmd = self.command.build_follow_up(&["--continue".to_string()]);
        let quoted_prompt = shell_quote_single(prompt);
        let cached = crate::profile::ProfileConfigs::get_cached();
        let profile = cached.get_profile("auggie");

        // Respect profile toggle: only allow best-effort --continue when explicitly enabled
        if let Some(p) = profile {
            if !p.auggie_use_continue_followup() {
                return Err(ExecutorError::FollowUpNotSupported(
                    "Auggie follow-up disabled (enable auggie_enable_continue_followup in profile)".to_string(),
                ));
            }
        } else {
            return Err(ExecutorError::FollowUpNotSupported(
                "Auggie profile not found".to_string(),
            ));
        }
        // Warn: --continue resumes the most recent Auggie session; VK session_id is ignored by Auggie CLI
        tracing::warn!(
            vk_session_id = %session_id,
            "Auggie follow-up uses --continue and ignores session_id; resuming most recent Auggie session"
        );


        let agent_cmd = Self::build_agent_cmd(&base_cmd, profile, &quoted_prompt);

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

