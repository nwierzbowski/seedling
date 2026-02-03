# Seedling AI Development Environment

Seedling is an AI development environment that orchestrates multiple Claude AI agents in a sophisticated terminal layout, managing hardware resources and AI processes for optimal performance.

## Overview

Seedling creates a complete AI development environment with:
- A llama-swap server running on port 8081
- A tmux session with 3 panes for different agent roles
- Claude Code instances configured for specific AI tasks (engineer, tester, auditor)
- GPU safety protocols and hardware management

## Features

### Terminal Layout
The system creates a sophisticated 3-pane terminal layout:
- **Pane 0 (Engineer)**: Main development environment
- **Pane 1 (Tester)**: Testing and validation tools
- **Pane 2 (Auditor)**: Monitoring and auditing capabilities

### Agent Configuration
Each pane gets a Claude Code instance with specific model configuration:
- Pane 0: `claude --model engineer`
- Pane 1: `claude --model tester`
- Pane 2: `claude --model auditor`

### Hardware Management
- NVIDIA GPU safety protocols (persistence mode, power limits, clock locking)
- Hardware resource monitoring and management
- GPU utilization tracking

## System Architecture

The seedling system consists of several modules:

### Process Management (`src/process.rs`)
Manages the lifecycle of AI processes including:
- Starting the llama-swap server with configuration
- Monitoring process health
- Graceful termination of all managed processes

### Hardware Management (`src/hardware.rs`)
Handles NVIDIA GPU configuration including:
- Safety locks for GPU access
- Persistence mode, power limit, and clock locking configurations
- Hardware resource monitoring

### Tmux Management (`src/tmux.rs`)
Manages terminal layouts and agent communication:
- Creates sophisticated 3-pane terminal layouts
- Injects Claude instances into specific tmux panes
- Handles session creation and pane management

## Usage

To start the seedling environment:

```bash
cargo run
```

This will:
1. Initialize hardware components with GPU safety protocols
2. Start the llama-swap AI server on port 8081
3. Create a tmux session with 3 panes
4. Open Claude Code instances in their respective panes

## Process Flow

The system follows this execution flow:
1. **Hardware Initialization**: GPU safety protocols are engaged
2. **AI Server Start**: llama-swap server is launched on port 8081
3. **Tmux Setup**: 3-pane terminal layout is created
4. **Agent Launch**: Claude instances are opened in their respective panes

## Configuration

The system uses:
- `llama-swap` with configuration file: `/home/nwier/models/llama-swap-config.yaml`
- Listening address: `0.0.0.0:8081`

## Hardware Safety Protocols

The system implements NVIDIA GPU safety protocols including:
- Persistence Mode (pm 1)
- Power Limit (300W)
- Clock Locking (2100MHz)

## Environment Variables

The system exports:
- `ANTHROPIC_BASE_URL=http://localhost:8081/v1`

This enables Claude instances to communicate with the local llama-swap server.

## System Requirements

- Rust 1.70+
- tmux
- NVIDIA GPU with nvidia-smi
- llama-swap binary at `/home/nwier/.local/bin/llama-swap`