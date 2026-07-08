use crate::a2a_interop::{build_message, A2ADispatcher, A2AError, A2AMessage, A2AResponse};
use crate::ancora_to_langchain::{expose_as_langchain_tool, AncoraAgentSpec};
use crate::crewai::{map_crewai_to_ancora, CrewAIAgent, CrewAIDefinition, CrewAITask};
/// Migration examples demonstrating how to move from each external framework
/// to Ancora. Each function returns a structured migration result that
/// integration tests can assert against.
use crate::langchain_tools::{import_langchain_tools, LangchainToolDef};
use crate::langgraph::{
    map_langgraph_to_stages, LangGraphDefinition, LangGraphEdge, LangGraphNode,
};
use crate::mcp_native::{McpParamDef, McpParamType, McpToolDef, McpToolRegistry};
use crate::openai_agents::{build_handoff, HandoffBridge, HandoffError, OpenAIAgentResult};
use crate::semantic_kernel::{import_sk_plugin, SKFunctionDef, SKFunctionParam, SKPluginDef};

pub struct MigrationResult {
    pub framework: String,
    pub items_migrated: usize,
    pub notes: String,
}

/// Example: migrate two LangChain tools into Ancora.
pub fn example_langchain_migration() -> MigrationResult {
    let defs = vec![
        LangchainToolDef {
            name: "web_search".into(),
            description: "Search the web".into(),
        },
        LangchainToolDef {
            name: "calculator".into(),
            description: "Evaluate math expressions".into(),
        },
    ];
    let tools = import_langchain_tools(defs);
    MigrationResult {
        framework: "langchain".into(),
        items_migrated: tools.len(),
        notes: format!(
            "Migrated tools: {}",
            tools
                .iter()
                .map(|t| t.tool_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Example: migrate a two-node LangGraph pipeline into Ancora stages.
pub fn example_langgraph_migration() -> MigrationResult {
    let graph = LangGraphDefinition {
        nodes: vec![
            LangGraphNode {
                id: "ingest".into(),
                label: "Ingest".into(),
            },
            LangGraphNode {
                id: "process".into(),
                label: "Process".into(),
            },
        ],
        edges: vec![LangGraphEdge {
            from: "ingest".into(),
            to: "process".into(),
        }],
        entry: "ingest".into(),
    };
    let stages = map_langgraph_to_stages(&graph).unwrap();
    MigrationResult {
        framework: "langgraph".into(),
        items_migrated: stages.len(),
        notes: "Stages ordered by topological sort".into(),
    }
}

/// Example: migrate a CrewAI crew definition into an Ancora crew plan.
pub fn example_crewai_migration() -> MigrationResult {
    let def = CrewAIDefinition {
        crew_name: "ops-crew".into(),
        agents: vec![
            CrewAIAgent {
                name: "ops-lead".into(),
                role: "Operations Lead".into(),
                goal: "Coordinate tasks".into(),
                backstory: "10 years of ops experience".into(),
            },
            CrewAIAgent {
                name: "analyst".into(),
                role: "Data Analyst".into(),
                goal: "Produce insights".into(),
                backstory: "Former data scientist".into(),
            },
        ],
        tasks: vec![
            CrewAITask {
                description: "Plan sprint".into(),
                assigned_to: "ops-lead".into(),
            },
            CrewAITask {
                description: "Analyse data".into(),
                assigned_to: "analyst".into(),
            },
        ],
    };
    let plan = map_crewai_to_ancora(def).unwrap();
    MigrationResult {
        framework: "crewai".into(),
        items_migrated: plan.members.len(),
        notes: format!(
            "Crew: {}, tasks: {}",
            plan.name,
            plan.task_assignments.len()
        ),
    }
}

/// Example: register MCP tools natively in Ancora.
pub fn example_mcp_migration() -> MigrationResult {
    let mut registry = McpToolRegistry::new();
    registry
        .register(McpToolDef {
            name: "read_file".into(),
            description: "Read file contents".into(),
            params: vec![McpParamDef {
                name: "path".into(),
                param_type: McpParamType::String,
                required: true,
                description: "File path".into(),
            }],
        })
        .unwrap();
    registry
        .register(McpToolDef {
            name: "list_dir".into(),
            description: "List directory".into(),
            params: vec![McpParamDef {
                name: "dir".into(),
                param_type: McpParamType::String,
                required: true,
                description: "Directory path".into(),
            }],
        })
        .unwrap();
    let count = registry.tool_names().len();
    MigrationResult {
        framework: "mcp".into(),
        items_migrated: count,
        notes: "MCP tools registered in Ancora native registry".into(),
    }
}

/// Example: expose an Ancora agent to LangChain.
pub fn example_expose_to_langchain() -> MigrationResult {
    let spec = AncoraAgentSpec {
        id: "report-gen".into(),
        display_name: "Report Generator".into(),
        capability_summary: "Generates structured reports from data".into(),
        endpoint: "http://localhost:9090/invoke".into(),
    };
    let desc = expose_as_langchain_tool(&spec);
    MigrationResult {
        framework: "langchain-expose".into(),
        items_migrated: 1,
        notes: format!("Exposed as LangChain tool: {}", desc.name),
    }
}

/// Example: A2A interop with a mock external agent.
pub fn example_a2a_migration() -> MigrationResult {
    let mut dispatcher = A2ADispatcher::new();
    dispatcher.register(
        "ext-llm",
        |msg: &A2AMessage| -> Result<A2AResponse, A2AError> {
            Ok(A2AResponse {
                responder_id: "ext-llm".into(),
                content: format!("processed: {}", msg.content),
                correlation_id: msg.correlation_id.clone(),
            })
        },
    );
    let msg = build_message("ancora-agent", "ext-llm", "What is the weather?");
    let resp = dispatcher.dispatch(&msg).unwrap();
    MigrationResult {
        framework: "a2a".into(),
        items_migrated: 1,
        notes: format!("A2A response: {}", resp.content),
    }
}

/// Example: OpenAI Agents SDK handoff bridge.
pub fn example_openai_agents_migration() -> MigrationResult {
    let mut bridge = HandoffBridge::new();
    bridge.register_agent(
        "classifier",
        |ctx: &str| -> Result<OpenAIAgentResult, HandoffError> {
            Ok(OpenAIAgentResult {
                agent_id: "classifier".into(),
                output: format!("classified: {}", ctx),
                finished: true,
            })
        },
    );
    let handoff = build_handoff("classifier", "needs classification", "raw user input");
    let result = bridge.execute_handoff(&handoff).unwrap();
    MigrationResult {
        framework: "openai-agents".into(),
        items_migrated: 1,
        notes: format!("Handoff output: {}", result.output),
    }
}

/// Example: Semantic Kernel plugin bridge.
pub fn example_sk_migration() -> MigrationResult {
    let plugin = SKPluginDef {
        plugin_name: "WritingPlugin".into(),
        functions: vec![
            SKFunctionDef {
                name: "draft_email".into(),
                description: "Draft an email".into(),
                params: vec![SKFunctionParam {
                    name: "tone".into(),
                    description: "Tone of the email".into(),
                    default_value: Some("professional".into()),
                }],
            },
            SKFunctionDef {
                name: "proofread".into(),
                description: "Proofread text".into(),
                params: vec![],
            },
        ],
    };
    let specs = import_sk_plugin(plugin).unwrap();
    MigrationResult {
        framework: "semantic-kernel".into(),
        items_migrated: specs.len(),
        notes: format!(
            "Imported SK functions: {}",
            specs
                .iter()
                .map(|s| s.qualified_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_examples_produce_results() {
        assert_eq!(example_langchain_migration().framework, "langchain");
        assert_eq!(example_langgraph_migration().framework, "langgraph");
        assert_eq!(example_crewai_migration().framework, "crewai");
        assert_eq!(example_mcp_migration().framework, "mcp");
        assert_eq!(example_expose_to_langchain().framework, "langchain-expose");
        assert_eq!(example_a2a_migration().framework, "a2a");
        assert_eq!(example_openai_agents_migration().framework, "openai-agents");
        assert_eq!(example_sk_migration().framework, "semantic-kernel");
    }
}
