# SignalFlow

[![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript&logoColor=white)](#) [![Rust](https://img.shields.io/badge/Rust-dea584?style=flat-square&logo=rust&logoColor=white)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Wire nodes together, hit Run, watch your data travel through the pipeline — no cloud, no accounts, no telemetry

SignalFlow is a visual dataflow programming desktop app. Think Unreal Blueprints meets ComfyUI — drag nodes onto a canvas, connect them with wires, and build data pipelines that run entirely on your machine. Ollama integration lets you drop local LLM nodes right into any flow.

## Features

- **Node canvas** — drag, connect, and configure nodes with a ReactFlow-powered graph editor; undo/redo everything
- **Rich node library** — file I/O, JSON parsing, HTTP requests, regex transforms, conditional routing, and Ollama prompt/chat nodes
- **Live execution** — watch data animate through your graph in real time; inline previews and a collapsible JSON inspector show exactly what's flowing
- **Pre-run validation** — misconfigured nodes are flagged before execution so you catch mistakes early
- **Flow management** — create, open, save, and delete multiple flows from a welcome screen or command palette
- **Persistent storage** — all flows auto-save to a local SQLite database in WAL mode; dark and light themes included

## Quick Start

### Prerequisites

- Node.js 18+
- pnpm 8+
- Rust stable toolchain (via [rustup](https://rustup.rs))
- macOS (v1.0 target; Linux/Windows support planned)
- [Ollama](https://ollama.com) for LLM nodes (optional)

### Installation

```bash
git clone https://github.com/saagpatel/SignalFlow.git
cd SignalFlow
pnpm install
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

## Tech Stack

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

## Architecture

The Rust backend owns graph execution: petgraph handles topological sort and cycle detection, while tokio drives async node evaluation. Each node type is a pure Rust function — no shared mutable state between nodes. The React frontend communicates with the backend exclusively via Tauri commands, keeping the execution engine fully decoupled from the UI. SQLite in WAL mode gives you safe concurrent reads while the execution engine writes live results.

## License

MIT
