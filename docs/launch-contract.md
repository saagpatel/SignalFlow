# SignalFlow Launch Contract

## Target

- Release target: `v1.0.0`
- Launch posture: macOS-first desktop release
- Distribution: manual GitHub Releases
- AI support: Ollama is a core supported feature

## Shipped Product Contract

- Visual workflow editor with palette, inspector, execution panel, welcome screen, save/open/delete, command palette, undo/redo, and SQLite-backed persistence
- Local execution engine in Rust with progress events, cancellation support, execution logs, and output inspection
- Node library that includes input, transform, output, control, AI, and code categories
- Settings that control Ollama endpoint, theme, and auto-save interval

## Must-Have Completion Gates

- Shared node catalog stays aligned across frontend and backend surfaces
- Save/load/new/restore-last-flow lifecycle is predictable and does not mark flows dirty unexpectedly
- Ollama endpoint setting is respected by health checks, model listing, and AI execution
- `pnpm verify` is the canonical local verification path
- CI and release workflows should converge on the same must-pass checks before `v1.0.0`

## Deliberate Non-Goals For First Production Release

- Auto-updater
- Cross-platform parity as a `v1.0.0` blocker beyond keeping shared code healthy
- Experimental Codex orchestration features as required delivery infrastructure
