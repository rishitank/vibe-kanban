# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Conventional Commits.

## [Unreleased]

### Added
- Auggie: Multi-MCP support via `mcp_config_paths` and backward-compatible `mcp_config_path`
- Auggie: Optional flags in profiles (`auggie_model`, `auggie_rules[]`, `auggie_augment_token_file`)
- Auggie: Deterministic command builder used in executor and tests
- Auggie: Best-effort follow-ups via `--continue` when `auggie_enable_continue_followup` is true
- Docs: README, npx-cli/README, AGENT.md FAQ, and CONTRIBUTING updates
- Dev: Example `dev_assets/profiles.json` variant with follow-ups enabled
- Tests: Command assembly ordering, dev wiring, and follow-up shape checks

### Changed
- Treat Auggie as MCP-capable; do not rely on a default MCP config path

### Notes
- Follow-ups: `--continue` resumes the most recent Auggie session and ignores VK `session_id`

