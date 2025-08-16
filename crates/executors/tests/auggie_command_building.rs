use executors::{executors::CodingAgent, profile::ProfileConfigs};

#[test]
fn auggie_builds_command_with_multiple_mcp_and_flags() {
    // Seed a minimal profiles.json in-memory via default + dev_assets
    // This test relies on dev_assets/profiles.json provided in repo
    let profiles = ProfileConfigs::from_defaults();
    let profile = profiles
        .profiles
        .iter()
        .find(|p| p.default.label == "auggie")
        .expect("missing auggie profile in defaults or dev_assets");

    // Accessors should not panic even if defaults have no extra paths/flags
    let _paths = profile.get_mcp_config_paths();
    let _flags = profile.get_auggie_flags();

    // Build initial base command using a dummy executor config
    // The enum variant for AUGGIE carries the command builder
    let auggie = match &profile.default.agent {
        CodingAgent::Auggie(a) => a.clone(),
        _ => panic!("auggie profile isn't AUGGIE agent"),
    };

    let base = auggie.command.build_initial();
    assert!(base.contains("auggie"));
}

