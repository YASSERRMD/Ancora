# MCP and A2A Interoperability (Rust)

## Exposing an agent as an MCP server

```rust
use ancora_core::{Runtime, AgentSpec};
use ancora_mcp::McpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions("You are a code review assistant.")
        .build();

    McpServer::new(rt, spec)
        .bind("0.0.0.0:8080")
        .serve()
        .await?;

    Ok(())
}
```

Any MCP-compatible client can now call your agent as a tool.

## Consuming an MCP tool from an agent

```rust
use ancora_mcp::McpToolBridge;

let mcp_tools = McpToolBridge::connect("http://other-service:8080").await?;

let spec = AgentSpec::builder()
    .model("llama3")
    .instructions("Use the remote tools to answer questions.")
    .tools(mcp_tools.into_tool_specs())
    .build();
```

## A2A (Agent-to-Agent) delegation

```rust
use ancora_a2a::{A2AClient, A2AServer};

// Server side
A2AServer::new(rt.clone(), spec.clone())
    .bind("0.0.0.0:9090")
    .serve()
    .await?;

// Client side
let client = A2AClient::connect("http://agent-b:9090").await?;
let result = client.delegate("Summarise this report", context).await?;
println!("{}", result);
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Edge deployment](edge-deployment.md)
