# Seedling Project - Top Level

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to its level in the project hierarchy. **Each `CLAUDE.md` in subdirectories should follow the same practice**: document only what's relevant at that level, pointing to deeper subdirectories as needed.

## Overview

This project is a Tauri-based desktop application for managing AI agent development workflows. It provides a unified interface for coordinating multiple Claude agents with hardware resource management (NVIDIA GPU) and process lifecycle control.

## High-Level Design & Best Practices

- **Modular Architecture**: Keep responsibilities separated. Core logic resides in `src-tauri/`, UI in `src/`.
- **Hierarchy Principle**: Each `CLAUDE.md` should only document its level; refer to subdirectory `CLAUDE.md` files for implementation details.
- **Safety First**: All hardware interactions (especially GPU) must include safety checks, locking, and graceful degradation.
- **Process Lifecycle Management**: Always monitor, restart on failure, and cleanly terminate processes.
- **Error Handling**: Use structured error types and ensure cleanup on panic or exit.
- **Documentation Sync**: Any major change to code must be reflected in the corresponding `CLAUDE.md` in the affected subdirectory.

## Project Structure

- **`src/`**: Frontend React components and UI logic.
  → See `src/CLAUDE.md` for component details, state management, and UI patterns.

- **`src-tauri/`**: Backend Rust logic including GPU management, process control, Tauri integration, and orchestration.
  → See `src-tauri/CLAUDE.md` for architecture, system interactions, and safety protocols.

- **`gen/`**: Generated code and build artifacts (frontend).
  → See `gen/CLAUDE.md` for details on generated content.

- **`public/`**: Static assets served directly by the application.

> ✅ **Note**: This file contains only high-level guidance. For implementation specifics, refer to the subdirectory-specific `CLAUDE.md` files.

