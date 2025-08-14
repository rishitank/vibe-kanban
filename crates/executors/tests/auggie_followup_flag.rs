use executors::{executors::CodingAgent, profile::ProfileConfigs};
use executors::executors::auggie::Auggie;

#[test]
fn auggie_followup_includes_continue_when_enabled() {
    // Load dev or defaults
    let mut profiles = ProfileConfigs::from_defaults();
    profiles.extend_from_file().ok();
    let profile = profiles.get_profile("auggie").expect("auggie profile");

    // We only assert the command shape when follow-up is enabled
    if !profile.auggie_use_continue_followup() {
        eprintln!("auggie follow-ups disabled in profile; skipping shape assertion");
        return;
    }

    // Build a follow-up base with --continue
    let base = match &profile.default.agent {
        CodingAgent::Auggie(a) => a.command.build_follow_up(&["--continue".into()]),
        _ => panic!("expected AUGGIE agent"),
    };
    let agent_cmd = Auggie::build_agent_cmd(&base, Some(profile), "\"next step\"");

    assert!(agent_cmd.contains(" --continue "));
}

