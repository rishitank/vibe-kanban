use executors::{executors::CodingAgent, profile::ProfileConfigs};
// Order: all --mcp-config paths (preserve profile order), then --model, then each --rules, then --augment-token-file,
use executors::executors::auggie::Auggie;

fn index_of(haystack: &str, needle: &str) -> usize {
    haystack.find(needle).unwrap_or(usize::MAX)
}

#[test]
fn auggie_build_agent_cmd_orders_flags_before_prompt() {
    let mut profiles = ProfileConfigs::from_defaults();
    profiles.extend_from_file().ok(); // pull dev_assets if present

    let profile = profiles.get_profile("auggie").expect("auggie profile");

    // Fake base and prompt
    let base = match &profile.default.agent {
        CodingAgent::Auggie(a) => a.command.build_initial(),
        _ => panic!("expected AUGGIE agent"),
    };
    let quoted_prompt = "\"hello world\"";

    let agent_cmd = Auggie::build_agent_cmd(&base, Some(profile), quoted_prompt);

    // Flag and prompt positions
    let mcpi_first = index_of(&agent_cmd, "--mcp-config ");
    let modeli = index_of(&agent_cmd, "--model ");
    let rulesi = index_of(&agent_cmd, "--rules ");
    let tokeni = index_of(&agent_cmd, "--augment-token-file ");
    let prompti = index_of(&agent_cmd, quoted_prompt);

    // Each present flag must appear before the prompt
    for idx in [mcpi_first, modeli, rulesi, tokeni] {
        if idx != usize::MAX {
            assert!(idx < prompti, "flags must come before prompt: {} < {}", idx, prompti);
        }
    }

    // Inter-flag ordering if flags exist: mcp-config(s) -> --model -> --rules -> --augment-token-file -> prompt
    if mcpi_first != usize::MAX && modeli != usize::MAX {
        assert!(mcpi_first < modeli, "mcp-config should come before --model");
    }
    if modeli != usize::MAX && rulesi != usize::MAX {
        assert!(modeli < rulesi, "--model should come before --rules");
    }
    if rulesi != usize::MAX && tokeni != usize::MAX {
        assert!(rulesi < tokeni, "--rules should come before --augment-token-file");
    }
}

