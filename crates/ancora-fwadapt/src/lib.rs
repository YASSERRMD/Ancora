pub mod a2a_interop;
pub mod ancora_to_langchain;
pub mod crewai;
pub mod examples;
/// ancora-fwadapt: Adapters for popular agent frameworks.
///
/// Provides import adapters (LangChain, LangGraph, CrewAI, MCP, OpenAI Agents SDK,
/// Semantic Kernel) and export adapters (Ancora -> LangChain), plus A2A interop
/// and runnable migration examples.
pub mod langchain_tools;
pub mod langgraph;
pub mod mcp_native;
pub mod openai_agents;
pub mod semantic_kernel;

#[cfg(test)]
mod tests;
