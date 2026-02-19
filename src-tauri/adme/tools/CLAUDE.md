# ADME Tools - src-tauri/adme/tools/

This directory contains tool implementations for the ADME agent system. Tools provide the interface between LLM agents and the underlying MeTTa symbolic logic system and memory store.

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to the `src-tauri/adme/tools/` level. For agent definitions, see `src-tauri/adme/`.

## Tool Categories

### Symbol Management
- **add_symbol.rs**: Adds a new symbol as a child of a parent symbol
- **get_symbols.rs**: Retrieves all available symbols from MeTTa

### Relationship Management
- **add_link.rs**: Links a child relationship to a parent relationship
- **relate.rs**: Creates a relationship between two symbols
- **bind_symbol.rs**: Binds two types at their intersections (placeholder)

### Memory Operations
- **store_memory.rs**: Stores new memories in the vector database via MeTTa
- **retrieve_memory.rs**: Retrieves similar memories using vector similarity search

## Common Patterns

- All tools implement the `rig::tool::Tool` trait
- MeTTa commands use `&` prefix for symbols (e.g., `&symbol`)
- Response parsing from MeTTa thread via crossbeam channels
- Error handling with custom `LookupError` type

## Implementation Notes

- Tools receive MeTTa channel endpoints in constructor
- Symbol grounding uses format: `!(add_symbol &parent "new_symbol" &parent)`
- Tool parameters defined via JSON schema in `definition()` method
