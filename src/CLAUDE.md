# Frontend React Code (src/)

This directory contains all frontend React components and UI logic for the Seedling application.

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to the `src/` level. For backend-related details (Tauri, Rust), see `src-tauri/CLAUDE.md`.

## Project Structure

### Main Application

**App.tsx** - Main application component that manages tab navigation between:
- Terminal Tab
- Agents Tab
- Settings Tab

### Components

See `src/components/CLAUDE.md` for component details.

- **TerminalTab.tsx** - Interactive terminal interface with Tauri integration
- **AgentsTab.tsx** - Agent management interface
- **SettingsTab.tsx** - Configuration panel
- **EngineerAgent.tsx**, **ReviewerAgent.tsx**, **TesterAgent.tsx** - Agent pane components

### Hooks

See `src/hooks/CLAUDE.md` for custom React hooks.

- **useShiftAHotkey.ts** - Hook for Shift+A keyboard shortcut

### Assets

See `src/assets/CLAUDE.md` for static resources.

- **react.svg** - React logo SVG

## Key Features

- Tab-based navigation system
- Integration with Tauri backend through event system
- Terminal emulator using xterm.js
- Responsive design
- Proper resource cleanup to prevent memory leaks
- Visual workflow diagrams using React Flow

### Dependencies Used
- React for component structure
- @xterm/xterm for terminal rendering
- @tauri-apps/api for Tauri integration
- reactflow for visual workflow diagrams
- CSS modules for styling

> ✅ **Note**: This file contains only high-level guidance. For implementation specifics, refer to the subdirectory-specific `CLAUDE.md` files.