# Seedling AI Development Environment - Tauri Module

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to the `src-tauri/` level. For frontend React details, see `src/CLAUDE.md`.

## What This Project Does

This Rust module provides the backend for the Seedling Tauri application, handling hardware resource management (NVIDIA GPU), AI process lifecycle control, and orchestration of agent workflows.
## Why This Project Exists

To provide a controlled, safe, and efficient desktop environment for developing and testing AI agents with Claude models—ensuring resource safety, process reliability, and coordinated agent workflows.
## How This System Works

### Architecture

- **Tauri Module** (`src-tauri/main.rs`):
  Entry point and orchestrator. Initializes and coordinates the Hardware, Process, and ADME modules. Provides the desktop UI and manages application state.

- **Hardware Module** (`src-tauri/hardware.rs`):
  Handles GPU access via `nvidia-smi` with sudo, enforcing safety: persistence mode, 300W power limit, and 2100MHz clock locking.

- **Process Module** (`src-tauri/process.rs`):
  Manages AI process lifecycle (start, monitor, restart, stop) using `tokio::process`, with retry logic and graceful shutdown.

- **ADME Module** (`src-tauri/adme/`):
  Contains the core agent system:
  - `agent.rs`: Agent trait for all ADME agents
  - `memory.rs`: Vector storage with similarity search for long-term memory
  - `planner.rs`: Language analyst that deconstructs natural language into symbolic graphs
  - `translator.rs`: Conversational response generator
  - `metta.rs`: MeTTa symbolic logic integration
  - `tools/`: Tool implementations for MeTTa and memory operations
  → See `src-tauri/adme/CLAUDE.md` for agent and tool details.

### Capabilities & Schemas

- **`src-tauri/capabilities/`**: Tauri capability definitions
  → See `src-tauri/capabilities/CLAUDE.md` for permission configuration.

- **`src-tauri/gen/`**: Auto-generated Tauri build artifacts
  → See `src-tauri/gen/CLAUDE.md` for generated code details.

### Execution Flow

1. Application starts and initializes Hardware, Process, and ADME managers.
2. Hardware module secures GPU with safety protocols.
3. AI processes (e.g., llama-swap) are launched and monitored.
4. Tauri UI renders agent panes and enables user interaction.
5. Agents run in coordinated environment; system shuts down cleanly.

### Key Features

- GPU safety via locking and power/clock limits
- Process resilience with automatic restarts
- Graceful shutdown with resource cleanup
- Centralized orchestration via Tauri UI
### Dependencies

- `tokio`, `anyhow`, `clap`, `sysinfo`, `nix`, `notify`, `regex`
  (Used for async, error handling, CLI, system ops, filesystem watching, and parsing)

> Note: Implementation details (e.g., command execution, parsing logic) are offloaded to respective subdirectory `CLAUDE.md` files.

