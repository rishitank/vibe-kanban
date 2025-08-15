use executors::{executors::CodingAgent, profile::ProfileConfigs};
// SPDX-License-Identifier: Apache-2.0

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
    let mcpi_last = agent_cmd.rfind("--mcp-config ").unwrap_or(usize::MAX);
    let modeli = index_of(&agent_cmd, "--model ");
    let rulesi = index_of(&agent_cmd, "--rules ");
    let tokeni = index_of(&agent_cmd, "--augment-token-file ");
    let prompti = index_of(&agent_cmd, quoted_prompt);
    assert!(prompti != usize::MAX, "quoted prompt must appear in the final command");

    // Each present flag must appear before the prompt
    for idx in [mcpi_first, modeli, rulesi, tokeni] {
        if idx != usize::MAX {
            assert!(idx < prompti, "flags must come before prompt: {idx} < {prompti}");
        }
    }
    if mcpi_last != usize::MAX {
        assert!(mcpi_last < prompti, "all --mcp-config flags must come before the prompt");
        if modeli != usize::MAX {
            assert!(mcpi_last < modeli, "all --mcp-config flags must come before --model");
        }
        if rulesi != usize::MAX {
            assert!(mcpi_last < rulesi, "all --mcp-config flags must come before --rules");
        }
        if tokeni != usize::MAX {
            assert!(mcpi_last < tokeni, "all --mcp-config flags must come before --augment-token-file");
        }
    }
}

