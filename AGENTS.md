# AGENTS.md

<!-- comm-contract:start -->

## Communication Contract

- Inherit global Codex communication and reporting rules from `/Users/d/.codex/AGENTS.override.md` and `/Users/d/.codex/policies/communication/BigPictureReportingV1.md`.
- Repo-specific instructions below add project constraints only; do not restate global voice or status-reporting rules here.
<!-- comm-contract:end -->

## Inherited Operating Rules

- Inherit global git, review/fix, testing, docs, skill-use, and reporting gates from `/Users/d/.codex/AGENTS.md` and active session instructions.
- Use `.codex/verify.commands` and `.codex/scripts/run_verify_commands.sh` as this repo's local verification authority when present.
- Keep the project-specific portfolio constraints below as the source of truth for runtime, privacy, and release risks.

<!-- portfolio-context:start -->
# Portfolio Context

## What This Project Is

SignalFlow is a local-first visual dataflow desktop app for node-based pipelines and local LLM workflows. It lets users build ReactFlow graphs, run node pipelines entirely on their machine, inspect live data movement, and persist flows to SQLite.

## Current State

The repo is active desktop product work. Existing local changes are PR-template metadata, so context recovery should stay documentation-only.

## Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Frontend | React 19 + TypeScript strict + Vite |
| Node graph | @xyflow/react (ReactFlow 12) |
| Styling | Tailwind CSS 4 |
| State | Zustand 5 + zundo (undo/redo) |
| Backend | Rust + tokio async runtime |
| Graph engine | petgraph (toposort, cycle detection) |
| Database | rusqlite (WAL mode, bundled SQLite) |
| Tests | Vitest |

## How To Run

```bash
# Development mode (hot reload)
pnpm tauri dev

# Run tests
pnpm test

# Production build
pnpm tauri build
```

## Known Risks

- Rust owns graph execution, topological sorting, cycle detection, and async node evaluation; keep execution decoupled from the UI.
- SQLite uses WAL mode for concurrent reads while runs write live results.
- Ollama nodes are local LLM workflow features; avoid cloud assumptions unless a provider feature is explicitly added.
- Keep PR-template drift separate from graph-engine or runtime changes.

## Next Recommended Move

Resolve PR-template drift separately, then verify graph validation, node execution, live previews, persistence, undo/redo, and local LLM nodes before shipping behavior changes.

<!-- portfolio-context:end -->
