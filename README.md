# SignalFlow

**Wire nodes. Build pipelines. Run locally.**

SignalFlow is a visual dataflow programming app for your desktop. Think Unreal Blueprints meets ComfyUI — drag nodes onto a canvas, connect them with wires, hit Run, and watch your data flow through the pipeline in real time.

Built with Tauri 2, React 19, and Rust. Everything runs on your machine. No cloud. No accounts. No telemetry.

---

## What Can You Do With It?

- Read files, parse JSON, filter arrays, merge data, write results
- Make HTTP requests and chain API calls together
- Run regex transforms, text templates, conditional routing
- Talk to local LLMs via Ollama — prompt nodes, chat nodes, the works
- Watch execution animate through your graph in real time
- Manage multiple flows — create, open, save, delete from the welcome screen or command palette
- See data flowing through your pipeline with inline node previews and a collapsible JSON inspector
- Catch mistakes early with pre-run validation that highlights misconfigured nodes
- Configure nodes with specialized editors — file pickers, model selectors, sliders, key-value editors
- Undo/redo everything, auto-save to SQLite, dark/light themes

## The Stack

| Layer | Tech |
|-------|------|
| Desktop Shell | **Tauri 2** |
| Frontend | **React 19** + TypeScript strict + Vite |
| Node Graph | **@xyflow/react** (ReactFlow 12) |
| Styling | **Tailwind CSS 4** |
| State | **Zustand 5** + zundo (undo/redo) |
| Backend | **Rust** + tokio async runtime |
| Graph Engine | **petgraph** (toposort, cycle detection) |
| Database | **rusqlite** (WAL mode, bundled SQLite) |
| LLM | **Ollama** (local models, streaming) |
| Command Palette | **cmdk** |

## Node Library

**18 node types** across 6 categories:

| Category | Nodes |
|----------|-------|
| Input | Text Input, Number Input, File Read, HTTP Request |
| Transform | JSON Parse, Text Template, Regex, Filter, Map, Merge, Split |
| Output | File Write, Debug |
| Control | Conditional (if/else branching), Code (JavaScript), Try/Catch, For Each |
| AI | LLM Prompt, LLM Chat |

Every node has typed ports (String, Number, Boolean, Array, Object, File, Any) with color-coded handles and connection validation. Nodes display inline config previews and output data directly on the canvas.

## Key Features

### Flow Management
- **Welcome screen** with recent flows list on startup
- **Auto-load** your last flow when you relaunch
- **Command palette** (Cmd+K) with New Flow, Open Flow, Save As, Delete Flow
- **Unsaved changes detection** with confirmation dialogs

### Smart Node Configuration
- **Config schema system** — each node type declares its fields, and the inspector renders specialized widgets automatically
- **File path picker** — native OS file dialogs for File Read/Write nodes
- **Model selector** — dropdown populated from your local Ollama models with availability detection
- **Sliders, dropdowns, key-value editors, checkboxes** — the right widget for each field

### Data Visibility
- **Inline output previews** on every node after execution (strings, arrays, objects, errors)
- **Collapsible JSON tree** in the inspector with type badges and copy-to-clipboard
- **50KB output cap** to keep the UI responsive on large payloads

### Pre-Run Validation
- Catches disconnected required inputs, empty config values, and orphan nodes
- **Clickable warnings** that select the problem node on the canvas
- Warnings shown as toasts and in the execution panel before logs

### Toast Notifications
- Success, error, warning, and info toasts for save, execution, validation, and flow management
- Auto-dismiss after 4 seconds, max 5 visible

### Advanced Features
- **JavaScript Expression Evaluation** — Filter, Map, and Conditional nodes support custom JavaScript expressions
  - Access `item` and `index` in Map/Filter nodes
  - Access `input` in Conditional and Code nodes
  - Full JavaScript expression support with syntax validation
- **Settings Panel** — Configure Ollama endpoint, theme (light/dark/auto), and auto-save interval
  - Test Ollama connection with live status indicator
  - System theme detection for auto mode
- **Error Context** — All error messages include node ID for precise debugging
- **Try/Catch Node** — Handle errors gracefully without failing the entire flow
- **For Each Node** — Iterate over arrays (basic implementation)
- **CI/CD Pipeline** — Automated testing on every push and PR

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 22+
- [pnpm](https://pnpm.io/) 9+
- [Rust](https://www.rust-lang.org/tools/install) stable
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS
- [Ollama](https://ollama.com/) (optional, for AI nodes)

### Run in Dev Mode

```bash
pnpm install
cargo tauri dev
```

### Build for Production

```bash
cargo tauri build
```

The `.dmg` (macOS) or installer lands in `src-tauri/target/release/bundle/`.

### Run Tests

```bash
pnpm test                      # Frontend (Vitest, 17 tests)
cd src-tauri && cargo test     # Backend (61 tests: 29 unit + 32 integration)
```

**Test Coverage:**
- HTTP integration tests (httpbin.org validation)
- File I/O tests (read/write, path traversal prevention)
- Database tests (CRUD, concurrency, WAL mode)
- Ollama integration tests (requires local Ollama, use `cargo test -- --ignored`)
- Expression evaluation tests
- Node executor tests

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+K` | Command palette |
| `Cmd+Enter` | Run flow |
| `Cmd+S` | Save |
| `Cmd+Z` / `Cmd+Shift+Z` | Undo / Redo |
| `Cmd+C` / `Cmd+V` | Copy / Paste nodes |
| `Cmd+D` | Duplicate selected |
| `Cmd+A` | Select all |
| `Backspace` | Delete selected |

## Project Structure

```
src/                    # React frontend
  components/
    canvas/             # Flow canvas, animated edges
    command-palette/    # Cmd+K command palette with flow management
    nodes/              # BaseNode + 12 specialized node components + DataPreview
    panels/             # Inspector, execution panel, config field editors (9 widgets)
    shared/             # Toast, ConfirmDialog
    toolbar/            # Top toolbar, status bar
    welcome/            # Welcome screen with recent flows
  stores/               # Zustand stores (flow, execution, UI, project)
  hooks/                # useExecution, useSaveFlow, useFlowManager, useToast
  lib/                  # Node registry, port types, flow validator, Tauri IPC

src-tauri/src/          # Rust backend
  engine/               # Graph builder, layer executor, execution context
  nodes/                # 16 node executors (input, transform, output, control, AI)
  db/                   # SQLite persistence (flows, executions, settings)
  ollama/               # Ollama HTTP client
  commands/             # Tauri IPC command handlers
```

## How Execution Works

1. Frontend validates the flow graph (checks connections, config, orphans)
2. Serializes the graph into a `FlowDocument` and sends to Rust
3. Rust builds a `petgraph::DiGraph`, runs toposort, detects cycles
4. Nodes are grouped into layers by dependency depth
5. Each layer executes sequentially; independent nodes within a layer could run in parallel
6. Progress events stream back to the frontend via Tauri Channels, including full output data
7. Nodes light up (blue = running, green = done, red = error) and edges animate
8. Output previews appear inline on each node; full data available in the inspector

Cancellation is instant — an `AtomicBool` flag is checked between each layer.

## License

MIT
