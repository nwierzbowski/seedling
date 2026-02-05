# Directory Structure Overview

## Frontend React Code (src/)

This project uses a Tauri-based desktop application with a React frontend. The src directory contains all the frontend code for the application.

### Main Application Structure:

**App.tsx** - Main application component that manages tab navigation between:
- Terminal Tab
- Agents Tab
- Settings Tab

**Components:**
1. **TerminalTab.tsx** - Interactive terminal interface with Tauri integration
   - Uses @xterm/xterm for terminal rendering
   - Connects to backend via Tauri event system
   - Implements proper cleanup and lifecycle management
   - Handles window resize events

2. **AgentsTab.tsx** - Agent management interface
   - Displays available Claude agents
   - Provides switching functionality between different agent personalities
   - Integrated with React Flow for visual agent collaboration diagrams

3. **SettingsTab.tsx** - Configuration panel
   - Settings for AI agent configurations
   - System settings and preferences

### Key Features of Frontend:

- Tab-based navigation system
- Integration with Tauri backend through event system
- Terminal emulator using xterm.js
- Responsive design
- Proper resource cleanup to prevent memory leaks
- Visual workflow diagrams using React Flow

### Dependencies Used:
- React for component structure
- @xterm/xterm for terminal rendering
- @tauri-apps/api for Tauri integration
- reactflow for visual workflow diagrams
- CSS modules for styling