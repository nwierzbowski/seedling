# Frontend React Code - src/

This directory contains all frontend React components and UI logic for the Seedling application.

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to the `src/` level. For backend-related details (Tauri, Rust), see `src-tauri/CLAUDE.md`.

## Purpose

 Provides the React UI for the Seedling desktop application, including tab navigation, terminal emulation, agent management, and settings configuration.

## Structure

- **Main Application**:
  - `App.tsx` - Main application component managing tab navigation between Terminal, Agents, and Settings tabs

- **Components** (see `components/CLAUDE.md`):
  - `TerminalTab.tsx` - Interactive terminal interface with Tauri integration
  - `AgentsTab.tsx` - Agent management interface
  - `SettingsTab.tsx` - Configuration panel
  - `EngineerAgent.tsx`, `ReviewerAgent.tsx`, `TesterAgent.tsx` - Agent pane components

- **Hooks** (see `hooks/CLAUDE.md`):
  - `useShiftAHotkey.ts` - Hook for Shift+A keyboard shortcut

- **Assets** (see `assets/CLAUDE.md`):
  - `react.svg` - React logo SVG placeholder

## Patterns

- Tab-based navigation using React Router or custom state
- Tauri event system integration for backend communication
- CSS modules for scoped component styling
- Responsive grid layouts for terminal and agent panes
- Proper resource cleanup to prevent memory leaks on unmount

## Dependencies

- React for component structure
- @xterm/xterm for terminal rendering
- @tauri-apps/api for Tauri integration
- reactflow for visual workflow diagrams
- CSS modules for styling

## Safety

- Event listener cleanup on component unmount to prevent memory leaks
- Tauri event handlers must be properly detached on unmount