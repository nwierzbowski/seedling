# AIDME AI Development Management Environment

## What This Project Does

AIDME is an AI development environment orchestration tool written in Rust designed to manage hardware resources and AI processes for Claude agents. It provides a sophisticated terminal-based interface with tmux pane layouts to support different agent roles (Engineer, Tester, Auditor) working together in what's called a "War Room" layout.

## Why This Project Exists

The project addresses the need for a structured, safe, and efficient environment for developing AI agents that work with Claude models. Key motivations include:

1. **Hardware Safety**: Managing NVIDIA GPU resources with safety protocols including persistence mode, power limits, and clock locking to prevent system instability
2. **Process Management**: Properly starting, monitoring, and terminating AI processes like llama-swap servers
3. **Environment Setup**: Creating a sophisticated terminal layout that enables multiple agents to work together in coordinated development workflows
4. **Resource Control**: Preventing GPU resource conflicts by ensuring proper locking mechanisms

## How This System Works

### Execution Flow

1. **Initialization**: The main application (`src-tauri/main.rs`) creates instances of all modules
2. **Hardware Setup**: The hardware manager engages safety protocols including GPU locking
3. **Process Startup**: AI processes (like llama-swap) are started with proper configuration
4. **Terminal Layout**: Tmux session is created with the sophisticated 3-pane layout
5. **Agent Launch**: Claude agents are launched in their respective panes with appropriate configurations

### Key Features

- **Safety Protocols**: Implements NVIDIA GPU safety measures to prevent system instability
- **Process Monitoring**: Continuously monitors AI processes for health and restarts when needed
- **Graceful Shutdown**: Proper cleanup of resources and processes when shutting down
- **Terminal Integration**: Sophisticated tmux pane layouts that enable multi-agent collaboration (in previous version)
- **Error Handling**: Comprehensive error handling with proper cleanup mechanisms

## Updating Documentation

When making major changes to the codebase, please remember to update the corresponding CLAUDE.md files:

1. If you modify any Rust modules in `src-tauri/`, update `src-tauri/CLAUDE.md`
2. If you modify React components in `src/`, update `src/CLAUDE.md`
3. After updating documentation, verify that the changes are consistent with the actual code implementation

This ensures that the documentation stays current with the codebase.

## Project Structure

For detailed documentation on the backend components, please refer to the CLAUDE.md files in subdirectories:

- **src/** - Contains the frontend application code for the user interface
  - Frontend components and UI logic
  - See `src/CLAUDE.md` for detailed documentation

- **src-tauri/** - Contains the main Rust backend implementation including hardware management, process control, and Tauri integration
  - See `src-tauri/CLAUDE.md` for detailed documentation

This is a sophisticated system designed to create a safe, controlled environment for AI agent development and testing with proper resource management, process lifecycle control, and terminal-based collaboration between multiple agent types.
