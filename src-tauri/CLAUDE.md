# Seedling AI Development Environment - Tauri Module

## What This Project Does

Seedling is an AI development environment orchestration tool written in Rust designed to manage hardware resources and AI processes for Claude agents. It provides a sophisticated desktop application interface using Tauri.

## Why This Project Exists

The project addresses the need for a structured, safe, and efficient environment for developing AI agents that work with Claude models. Key motivations include:

1. **Hardware Safety**: Managing NVIDIA GPU resources with safety protocols including persistence mode, power limits, and clock locking to prevent system instability
2. **Process Management**: Properly starting, monitoring, and terminating AI processes like llama-swap servers
3. **Environment Setup**: Creating a sophisticated desktop application interface that enables multiple agents to work together in coordinated development workflows
4. **Resource Control**: Preventing GPU resource conflicts by ensuring proper locking mechanisms

## How This System Works

### Architecture Overview

The system is organized into three main modules that work together:

1. **Hardware Module** (`src-tauri/hardware.rs`)
   - Manages NVIDIA SMI locks for GPU access
   - Implements safety protocols including persistence mode, power limits (300W), and clock locking (2100MHz)
   - Provides GPU monitoring capabilities
   - Uses `nvidia-smi` commands with sudo privileges for hardware configuration

2. **Process Module** (`src-tauri/process.rs`)
   - Manages AI processes like llama-swap server lifecycle
   - Handles graceful startup, monitoring, and termination of processes
   - Implements process restart logic with maximum retry limits
   - Uses `tokio::process` for async process management

3. **Tauri Module** (`src-tauri/main.rs`)
   - Creates sophisticated desktop application interface using Tauri
   - Manages application state including hardware and process managers
   - Handles backend communication with frontend

### Execution Flow

1. **Initialization**: The main application (`src-tauri/main.rs`) creates instances of all modules
2. **Hardware Setup**: The hardware manager engages safety protocols including GPU locking
3. **Process Startup**: AI processes (like llama-swap) are started with proper configuration
4. **Application Interface**: Tauri-based desktop application is created with the sophisticated interface
5. **Agent Launch**: Claude agents are launched in their respective panes with appropriate configurations

### Key Features

- **Safety Protocols**: Implements NVIDIA GPU safety measures to prevent system instability
- **Process Monitoring**: Continuously monitors AI processes for health and restarts when needed
- **Graceful Shutdown**: Proper cleanup of resources and processes when shutting down
- **Error Handling**: Comprehensive error handling with proper cleanup mechanisms

### Dependencies

The project uses several Rust crates including:
- `tokio` for async runtime
- `anyhow` for error handling
- `clap` for command-line argument parsing
- `sysinfo`, `nix` for system operations
- `notify` for filesystem watching
- `regex` for parsing output

This is a sophisticated system designed to create a safe, controlled environment for AI agent development and testing with proper resource management, process lifecycle control, and desktop-based collaboration between multiple agent types.