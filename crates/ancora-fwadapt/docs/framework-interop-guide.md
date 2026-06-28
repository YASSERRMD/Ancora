# Framework Interoperability Guide

ancora-fwadapt provides adapters so Ancora agents can consume tools from — and
expose capabilities to — the most popular agent frameworks without rewriting
business logic.

## Supported frameworks

| Framework | Direction | Module |
|---|---|---|
| LangChain | Import tools | `langchain_tools` |
| LangGraph | Import graph topology | `langgraph` |
| CrewAI | Import crew/role definitions | `crewai` |
| MCP (Model Context Protocol) | Import tool definitions | `mcp_native` |
| LangChain | Expose Ancora agent | `ancora_to_langchain` |
| A2A (Agent-to-Agent) | Bidirectional interop | `a2a_interop` |
| OpenAI Agents SDK | Handoff bridge | `openai_agents` |
| Semantic Kernel | Plugin bridge | `semantic_kernel` |

## Design principles

- No external crate dependencies beyond `std`.
- All adapters are pure data transformations or in-process function dispatchers.
- Network I/O is the caller's responsibility; adapters produce descriptors and
  route to registered handlers.
- Errors are typed; no panics in library code.

## Quick start

```rust
use ancora_fwadapt::langchain_tools::{import_langchain_tools, LangchainToolDef};

let defs = vec![LangchainToolDef {
    name: "search".into(),
    description: "Web search".into(),
}];
let tools = import_langchain_tools(defs);
let result = tools[0].run("Rust async runtimes").unwrap();
```
