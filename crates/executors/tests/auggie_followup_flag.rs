use executors::{executors::CodingAgent, profile::ProfileConfigs};
use executors::executors::auggie::Auggie;

#[test]
fn auggie_followup_includes_continue_when_enabled() {
    // Load dev or defaults
    let mut profiles = ProfileConfigs::from_defaults();
    profiles.extend_from_file().ok();
    let profile = profiles.get_profile("auggie").expect("auggie profile");

    // Build a follow-up base with --continue
    let base = match &profile.default.agent {
        CodingAgent::Auggie(a) => a.command.build_follow_up(&["--continue".into()]),
        _ => panic!("expected AUGGIE agent"),
    };
    let agent_cmd = Auggie::build_agent_cmd(&base, Some(profile), "\"next step\"");

    // If follow-ups are disabled, the builder still produces a valid shape; spawn_follow_up enforces the flag.
    assert!(agent_cmd.contains(" --continue "));

    // Also assert that flags appear before the prompt as in normal spawn
    let prompti = agent_cmd.find("\"next step\"").unwrap();
    let conti = agent_cmd.find(" --continue ").unwrap();
    assert!(conti < prompti);
}

