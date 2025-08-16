# Contributing

Thanks for your interest in contributing to Vibe Kanban!

## End-to-end tips: Auggie follow-ups

Vibe Kanban supports an optional best-effort follow-up mode for the Auggie CLI.

- What it does: uses `--continue` to resume the most recent Auggie session
- Caveat: the Auggie CLI ignores VK’s session_id and resumes the last saved session
- How to enable:
  1. In `dev_assets/profiles.json`, use the `with-extra-mcp` variant which has `"auggie_enable_continue_followup": true`
  2. Or add `"auggie_enable_continue_followup": true` to your own Auggie profile/variant in `profiles.json`
- How to verify:
  - Run backend tests: `cargo test -p executors --tests -- --nocapture`
  - Tail logs to see the info/warn lines about follow-ups

Once the Auggie CLI supports explicit session targeting (e.g., `--session-id`), we’ll wire it and remove the caveat above.

## Development workflow

- Create a feature branch from `main`
- Keep PRs focused and small; add tests for new logic
- Run `npm run check` before pushing (Rust + TypeScript checks)
- Update docs (README/AGENT.md) if user-facing behavior changes
- Use conventional commit messages

## Code of Conduct

Be respectful and constructive. See CODE_OF_CONDUCT.md (coming soon).

