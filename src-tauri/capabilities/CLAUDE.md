# Tauri Capabilities - src-tauri/capabilities/

This directory contains Tauri capability files that define what platform APIs and system resources the application is allowed to use.

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to the `src-tauri/capabilities/` level. For Tauri configuration, see `src-tauri/CLAUDE.md`.

## Capabilities

- **default.json**: Default capability set defining allowed Tauri APIs

## Patterns

- Capability files use JSON schema for permission definitions
- Permissions control access to Tauri APIs (file system, processes, etc.)
- Capability names map to build configuration in `tauri.conf.json`
