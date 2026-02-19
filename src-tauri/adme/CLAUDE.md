# ADME Core - src-tauri/adme/

This directory contains the core ADME (AIDME) agent system implementation, including agent traits, memory management, planning, and symbolic logic integration via MeTTa.

## 📝 File Scope

This `CLAUDE.md` contains only details pertinent to the `src-tauri/adme/` level. For Tauri API integration and hardware management, see `src-tauri/CLAUDE.md`.

## Structure

- **agent.rs**: Agent trait defining the interface for all ADME agents
- **memory.rs**: Vector storage with similarity search for long-term memory
- **planner.rs**: Language analyst that deconstructs natural language into symbolic graphs
- **translator.rs**: Conversational response generator for user-facing output
- **metta.rs**: MeTTa symbolic logic integration and thread management

## Tools (src-tauri/adme/tools/)

See `src-tauri/adme/tools/CLAUDE.md` for tool implementations.

- **get_symbols.rs**: Query all available symbols from MeTTa
- **add_symbol.rs**: Add new symbols to the knowledge graph
- **add_link.rs**: Link relationships in the knowledge graph
- **bind_symbol.rs**: Bind two types at intersections
- **relate.rs**: Create relationships between symbols
- **store_memory.rs**: Store new memories in vector database
- **retrieve_memory.rs**: Retrieve similar memories via vector search

## Agent Roles

- **Planner**: Analyzes language and builds symbolic representations
- **Memory**: Stores and retrieves past conversations via RAG
- **Translator**: Generates human-like conversational responses

## Patterns

- Async agent prompting with Ollama
- Vector embeddings for memory similarity search
- MeTTa thread spawn with crossbeam channels for command/response
- Tool-based interaction with symbolic logic system
