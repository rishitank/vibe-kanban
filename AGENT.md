# Agent Guide

## Commands

Check package.json for available scripts

## Architecture

- **Full-stack Rust + React monorepo** with pnpm workspace
- **Backend**: Rust/Axum API server (port 3001) with Tokio async runtime
- **Frontend**: React 18 + TypeScript + Vite (port 3000) with shadcn/ui components
- **Shared**: Common TypeScript types in `/shared/types.ts`
- **API**: REST endpoints at `/api/*` proxied from frontend to backend in dev

## Code Style

- **Rust**: Standard rustfmt, snake_case, derive Debug/Serialize/Deserialize
- **TypeScript**: Strict mode, @/ path aliases, interfaces over types
- **React**: Functional components, hooks, Tailwind classes
- **Imports**: Workspace deps, @/ aliases for frontend, absolute imports
- **Naming**: PascalCase components, camelCase vars, kebab-case files

# Managing Shared Types Between Rust and TypeScript

ts-rs allows you to derive TypeScript types from Rust structs/enums. By annotating your Rust types with #[derive(TS)] and related macros, ts-rs will generate .ts declaration files for those types.
When making changes to the types, you can regenerate them using `npm run generate-types`
Do not manually edit shared/types.ts, instead edit backend/src/bin/generate_types.rs

# Working on the frontend AND the backend

When working on any task that involves changes to the backend and the frontend, start with the backend. If any shared types need to be regenerated, regenerate them before starting the frontend changes.

# Testing your work

`npm run check` - runs cargo and tsc checks

# Backend data models

SQLX queries should be located in backend/src/models/\*
Use getters and setters instead of raw SQL queries where possible.


# Auggie vs other coding agents

Vibe Kanban treats Auggie as a first-class coding agent alongside Claude, Gemini, Codex, Amp, Cursor, and Opencode. Highlights:

- MCP parity:
  - Auggie supports repeatable `--mcp-config` flags; VK passes one or many based on profiles
  - Other agents use known config paths; VK manages those files where appropriate
- Profile-driven flags (Auggie only, optional):
  - `auggie_model`, `auggie_rules[]`, `auggie_augment_token_file` mapped to CLI flags
- UX differences:
  - Auggie defaults to `--print` (one-shot), ideal for CI and non-interactive tasks
  - Interactive mode is available by removing `--print`, with logs normalized like others
- When to prefer Auggie:
  - You need multi-MCP composition without editing agent-owned config files
  - You want explicit CLI-driven config and reproducible commands in logs
- When others shine:
  - If you rely on their native GUIs/flows or ecosystem-specific tools
  - If your team already centralizes config in their canonical files VK manages

Bottom line: Auggie is not universally “superior,” but for CLI-first, MCP-rich workflows, it’s extremely capable and now fully supported with parity features in VK.
