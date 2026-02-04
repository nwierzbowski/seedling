# Seedling - AI Development Environment with Terminal UI

A sophisticated Tauri-based desktop application that orchestrates multiple Claude AI agents in an interactive terminal environment.

## Project Structure

```
seedling/
├── src/                    # Frontend (JavaScript/React)
│   ├── index.html         # HTML entry point
│   ├── main.jsx           # React entry point
│   ├── App.jsx            # Main application component
│   ├── Terminal.jsx       # Terminal UI component
│   ├── App.css
│   ├── Terminal.css
│   └── index.css
├── src-tauri/             # Backend (Rust)
│   ├── main.rs            # Tauri application entry point
│   ├── hardware.rs        # GPU/Hardware management
│   ├── process.rs         # Process management
│   ├── tauri_interface.rs # Tauri-specific interface
│   └── ...other modules
├── Cargo.toml             # Rust project configuration
├── package.json           # Node.js dependencies
├── vite.config.js         # Vite frontend build configuration
├── tauri.conf.json        # Tauri application configuration
└── build.rs               # Tauri build script
```

## Getting Started

### Prerequisites

- Rust (for building the backend)
- Node.js and npm (for building the frontend)
- GTK development libraries (on Linux)

### Installation

```bash
# Install Rust dependencies (backend)
cargo build

# Install Node.js dependencies (frontend)
npm install
```

### Development

To run the application in development mode with hot-reload:

```bash
# Terminal 1: Start the frontend dev server
npm run dev

# Terminal 2 (in another window): Start the Tauri application
seedling
```

The frontend dev server will start on `http://localhost:5173` and Tauri will connect to it for development.

### Building for Production

```bash
npm run build
cargo install --path .
```

This will create an optimized production build and install the `seedling` binary.

## Features

### Frontend Interface
- **Status Panel**: Displays system status including GPU and process information
- **Agent Control**: Switch between Engineer, Tester, and Auditor agents
- **Terminal Interface**: Interactive terminal with command input and agent output
  - Real-time command execution
  - Agent-specific responses
  - Terminal cursor animation
  - Clear and pause controls

### Agent Types

1. **Engineer Agent**: Handles development tasks, system architecture, and implementation
2. **Tester Agent**: Manages test coverage and quality assurance
3. **Auditor Agent**: Reviews code for security and best practices

### Backend Features
- Hardware safety protocols (GPU management)
- Process management and monitoring
- Tauri inter-process communication (IPC)
- Agent command routing and execution

## Tauri Commands

The backend exposes the following commands to the frontend:

- `get_status`: Retrieve current system status
- `switch_agent`: Switch between active agents
- `execute_command`: Execute a command in the selected agent context

## Keyboard Shortcuts

- `Ctrl+C` in the terminal window: Gracefully shutdown the application
- `Enter` in the terminal input: Execute the entered command
- Click agent buttons: Switch active agent

## Performance

- Frontend builds with Vite for fast development iteration
- Tauri provides efficient window management and IPC
- Rust backend for safe, performant system operations

## Development Notes

The project uses:
- **React** for the UI framework
- **Vite** for frontend build tooling
- **Tauri 2.0** for desktop application framework
- **Rust** for backend and system operations

Terminal styling follows a classic dark terminal theme with green text and black background for optimal readability.

## Troubleshooting

### Port 5173 already in use

If port 5173 is already in use, you can change it in `vite.config.js`:

```javascript
export default defineConfig({
  server: {
    port: 5174, // Change to a different port
  }
})
```

### Connection refused errors

Make sure the frontend dev server is running on port 5173 before starting the Tauri application.

### Build issues

For fresh builds, try:

```bash
rm -rf target node_modules dist
cargo clean
npm install
cargo build
```
