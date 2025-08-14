use executors::{executors::CodingAgent, profile::ProfileConfigs};

#[test]
fn auggie_builds_all_flags_from_dev_profiles() {
    // Load from dev_assets via ProfileConfigs::load path resolution
    let mut profiles = ProfileConfigs::from_defaults();
    // Merge user/dev profiles to include dev_assets/profiles.json
    profiles.extend_from_file().ok();

    let profile = profiles
        .get_profile("auggie")
        .expect("missing auggie profile");

    // Assert multi-MCP collected
    let paths = profile.get_mcp_config_paths();
    assert!(paths.len() >= 2, "expected at least two MCP paths");

    // Assert optional flags are present from dev_assets example
    let flags = profile.get_auggie_flags();
    // At minimum we expect model and token flags in the dev example
    assert!(flags.iter().any(|f| f.starts_with("--model ")));
    assert!(flags.iter().any(|f| f.starts_with("--augment-token-file ")));

    // Ensure base command is auggie
    let auggie = match &profile.default.agent {
        CodingAgent::Auggie(a) => a.clone(),
        _ => panic!("auggie profile isn't AUGGIE agent"),
    };
    let base = auggie.command.build_initial();
    assert!(base.contains("auggie"));
}

