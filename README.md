# SignalFlow

[![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript&logoColor=white)](#) [![Rust](https://img.shields.io/badge/Rust-dea584?style=flat-square&logo=rust&logoColor=white)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Wire nodes together, hit Run, watch your data travel through the pipeline — no cloud, no accounts, no telemetry

SignalFlow is a visual dataflow programming desktop app. Think Unreal Blueprints meets ComfyUI — drag nodes onto a canvas, connect them with wires, and build data pipelines that run entirely on your machine. Ollama integration lets you drop local LLM nodes right into any flow.

## Features

- **Node canvas** — drag, connect, and configure nodes with a ReactFlow-powered graph editor; undo/redo node and edge changes
- **Rich node library** — file I/O, JSON parsing, HTTP requests, regex transforms, conditional routing, and Ollama prompt/chat nodes
- **Live execution** — watch data animate through your graph in real time; inline previews and a collapsible JSON inspector show exactly what's flowing
- **Pre-run validation** — misconfigured nodes are flagged before execution so you catch mistakes early
- **Flow management** — create, open, save, and delete multiple flows from a welcome screen or command palette
- **Persistent storage** — dirty, non-empty flows auto-save from the editor to a local SQLite database in WAL mode when auto-save is enabled; dark and light themes included

## Quick Start

### Prerequisites

- Node.js 22.22.2+ in the Node 22 line (required by the locked jsdom 30 test environment; CI uses Node 22)
- pnpm 10.31.0 (the CI version)
- Rust stable toolchain (via [rustup](https://rustup.rs))
- macOS (v1.0 target; Linux/Windows support planned)
- [Ollama](https://ollama.com) for LLM nodes (optional)

### Installation

```bash
git clone https://github.com/saagpatel/SignalFlow.git
cd SignalFlow
pnpm install --frozen-lockfile
```

### Usage

```bash
# Development mode (hot reload)
pnpm tauri dev

# Run tests
pnpm test

# Production build
pnpm tauri build
```

## Verification

Run frontend commands from the root using the committed `pnpm-lock.yaml`.
Installation runs the Husky `prepare` script to configure Git hooks; use a
standalone development clone rather than a runtime-pinned/shared checkout.
A focused, offline check is:

```bash
pnpm test src/lib/connectionValidator.test.ts
```

For the broader required lane, use `pnpm verify`. The authoritative commands are
[`.codex/verify.commands`](.codex/verify.commands); the runner starts each command
from the repository root. It covers Git guards, lint, types, tests/coverage,
frontend build, docs, Rust checks, and performance measurements. The local
secret guard requires `gitleaks` on `PATH` with support for
`gitleaks protect --staged --redact`; without it, local `pnpm verify` and commit
hooks fail. Install a compatible gitleaks CLI before using that lane. On a feature
branch, stage only your own changes before the Git guards. Required CI budgets
and diff coverage remain separate gates; local measurements are not a waiver.

For focused Rust checks, run `cargo test --lib` from `src-tauri`. The full
`cargo test` suite uses local HTTP/file fixtures but also attempts an HTTP
request to an external hostname and non-ignored Ollama calls; ignored Ollama
tests add further live coverage. Do not run the full suite, enable ignored tests, or use live
LLM/HTTP/file nodes merely to verify documentation. Native checks need the stable
Rust toolchain with rustfmt/Clippy and macOS Tauri build prerequisites. CI also
builds and validates a macOS bundle; frontend `pnpm build` alone does not do that.

When canvas, ports, previews, persistence, or report behavior changes, use
`pnpm tauri dev` with a disposable OS profile/database and synthetic flows.
Check the changed workflow, loading/empty/error states, and keyboard interaction.
`pnpm dev` alone cannot validate native database or dialog behavior. Browser/UI
checks are conditional; keep real files, existing projects, and live providers
out of fixtures.

## Tech Stack

| Layer         | Technology                           |
| ------------- | ------------------------------------ |
| Desktop shell | Tauri 2                              |
| Frontend      | React 19 + TypeScript strict + Vite  |
| Node graph    | @xyflow/react (ReactFlow 12)         |
| Styling       | Tailwind CSS 4                       |
| State         | Zustand 5 + zundo (undo/redo)        |
| Backend       | Rust + tokio async runtime           |
| Graph engine  | petgraph (toposort, cycle detection) |
| Database      | rusqlite (WAL mode, bundled SQLite)  |
| Tests         | Vitest                               |

## Architecture

The Rust backend owns graph execution: petgraph handles topological sort and cycle detection, while tokio drives async node evaluation. Node types implement the Rust `NodeExecutor` trait and share an `ExecutionContext` containing outputs, cancellation state, and the current node ID; file and HTTP nodes perform I/O. The React frontend invokes Tauri commands and receives execution progress via a Tauri IPC channel, keeping the execution engine decoupled from the UI. SQLite uses WAL mode with a single mutex-protected connection; execution results are saved after a run completes for flows with an ID.

## Release Docs

- [Launch Contract](docs/launch-contract.md)
- [Release Readiness](docs/release-readiness.md)

## License

MIT
