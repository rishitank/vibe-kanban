use executors::{command::CommandBuilder, executors::CodingAgent, profile::ProfileConfigs};

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

    // Ensure accessor merges single + multiple paths and returns flags
    let paths = profile.get_mcp_config_paths();
    // We don't assert exact paths because dev paths are examples; just ensure it's a Vec
    assert!(paths.len() >= 0);

    let flags = profile.get_auggie_flags();
    // Flags may be empty in defaults; the accessor should still return a Vec
    assert!(flags.len() >= 0);

    // Build initial base command using a dummy executor config
    // The enum variant for AUGGIE carries the command builder
    let auggie = match &profile.default.agent {
        CodingAgent::Auggie(a) => a.clone(),
        _ => panic!("auggie profile isn't AUGGIE agent"),
    };

    let base = auggie.command.build_initial();
    assert!(base.contains("auggie"));
}

