# Seedling AI Development Environment

## What This Project Does

Seedling is an AI development environment orchestration tool written in Rust designed to manage hardware resources and AI processes for Claude agents. It provides a sophisticated terminal-based interface with tmux pane layouts to support different agent roles (Engineer, Tester, Auditor) working together in what's called a "War Room" layout.

## Why This Project Exists

The project addresses the need for a structured, safe, and efficient environment for developing AI agents that work with Claude models. Key motivations include:

1. **Hardware Safety**: Managing NVIDIA GPU resources with safety protocols including persistence mode, power limits, and clock locking to prevent system instability
2. **Process Management**: Properly starting, monitoring, and terminating AI processes like llama-swap servers
3. **Environment Setup**: Creating a sophisticated terminal layout that enables multiple agents to work together in coordinated development workflows
4. **Resource Control**: Preventing GPU resource conflicts by ensuring proper locking mechanisms

## How This System Works

### Architecture Overview

The system is organized into three main modules that work together:

1. **Hardware Module** (`src/hardware.rs`)
   - Manages NVIDIA SMI locks for GPU access
   - Implements safety protocols including persistence mode, power limits (300W), and clock locking (2100MHz)
   - Provides GPU monitoring capabilities
   - Uses `nvidia-smi` commands with sudo privileges for hardware configuration

2. **Process Module** (`src/process.rs`)
   - Manages AI processes like llama-swap server lifecycle
   - Handles graceful startup, monitoring, and termination of processes
   - Implements process restart logic with maximum retry limits
   - Uses `tokio::process` for async process management

3. **Tmux Module** (`src/tmux.rs`)
   - Creates sophisticated 3-pane terminal layouts using tmux
   - Implements a "War Room" layout with Engineer, Tester, and Auditor panes
   - Handles agent communication by sending commands to specific panes
   - Uses tmux send-keys functionality for command injection

### Execution Flow

1. **Initialization**: The main application (`src/main.rs`) creates instances of all modules
2. **Hardware Setup**: The hardware manager engages safety protocols including GPU locking
3. **Process Startup**: AI processes (like llama-swap) are started with proper configuration
4. **Terminal Layout**: Tmux session is created with the sophisticated 3-pane layout
5. **Agent Launch**: Claude agents are launched in their respective panes with appropriate configurations

### Key Features

- **Safety Protocols**: Implements NVIDIA GPU safety measures to prevent system instability
- **Process Monitoring**: Continuously monitors AI processes for health and restarts when needed
- **Graceful Shutdown**: Proper cleanup of resources and processes when shutting down
- **Terminal Integration**: Sophisticated tmux pane layouts that enable multi-agent collaboration
- **Error Handling**: Comprehensive error handling with proper cleanup mechanisms

### Dependencies

The project uses several Rust crates including:
- `tokio` for async runtime
- `anyhow` for error handling
- `clap` for command-line argument parsing
- `sysinfo`, `nix` for system operations
- `notify` for filesystem watching
- `regex` for parsing output

This is a sophisticated system designed to create a safe, controlled environment for AI agent development and testing with proper resource management, process lifecycle control, and terminal-based collaboration between multiple agent types.

## Future Development: Tauri-Based Interface

We are transitioning from the tmux-based system to a Tauri-based desktop application that will enable swapping between different agent personalities in a single window. This represents a significant architectural shift:

### New Architecture Overview

1. **Frontend**: Tauri-based desktop application using web technologies (Rust + WebAssembly)
2. **Backend**: Rust backend for system management, process control, and hardware interaction
3. **Agent Switching**: Ability to swap between different Claude personalities within a single window
4. **Integrated Interface**: Unified UI that can display multiple agent roles simultaneously

### Key Features of New Implementation

- Single-window interface with agent switching capability
- Seamless integration with Claude API endpoints
- Cross-platform desktop application
- Modern web-based UI components
- Persistent agent configurations
- Real-time agent status monitoring

## Development Roadmap

### Phase 1: Infrastructure Setup (Completed)
- [ ] Project structure migration to Tauri
- [ ] Rust backend architecture design
- [ ] Basic Tauri application scaffolding
- [ ] Build system configuration

### Phase 2: Core System Components
- [ ] Hardware Manager Implementation
- [ ] Process Management System
- [ ] Agent Personality Manager
- [ ] Configuration Storage System

### Phase 3: User Interface Development
- [ ] Main Application Window Design
- [ ] Agent Switching Interface
- [ ] Terminal/Console Component
- [ ] Status Monitoring Dashboard
- [ ] Settings and Preferences Panel

### Phase 4: Integration and Testing
- [ ] API Integration with Claude services
- [ ] Hardware Safety Protocol Implementation
- [ ] Process Lifecycle Management
- [ ] Cross-platform Testing
- [ ] Performance Optimization

### Phase 5: Finalization and Documentation
- [ ] Complete User Guide
- [ ] API Documentation
- [ ] Deployment Instructions
- [ ] Release Notes