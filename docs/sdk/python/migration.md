# Migration from LangChain, LangGraph, and CrewAI (Python)

## From LangChain

| LangChain concept | Ancora equivalent |
|-------------------|-------------------|
| `LLM` / `ChatModel` | `AgentSpec.model` string |
| `Tool` | `@registry.tool(description=...)` decorator |
| `AgentExecutor` | `Runtime.run(spec, prompt)` |
| `PromptTemplate` | `AgentSpec.instructions` string with f-string variables |
| `Memory` | `StoringTransport` + `SqliteStore` |
| `Callback` | Iterate `RunHandle.events()` |

### Before (LangChain)

```python
from langchain.agents import AgentExecutor, create_tool_calling_agent
from langchain_community.llms import Ollama

llm = Ollama(model="llama3")
tools = [get_weather_tool]
agent = create_tool_calling_agent(llm, tools, prompt_template)
executor = AgentExecutor(agent=agent, tools=tools)
result = executor.invoke({"input": "What is the weather in Cairo?"})
```

### After (Ancora)

```python
from ancora import Runtime, AgentSpec, ToolRegistry

registry = ToolRegistry()

@registry.tool(description="Return the weather for a city.")
def get_weather(city: str) -> str:
    return f"{city}: 22 C"

rt = Runtime()
spec = AgentSpec("llama3", "Answer weather questions.", tools=registry)
result = rt.run(spec, "What is the weather in Cairo?")
print(result.output)
```

## From LangGraph

| LangGraph concept | Ancora equivalent |
|-------------------|-------------------|
| `StateGraph` | `GraphSpec` |
| `add_node` | `GraphNode` in `nodes` list |
| `add_edge` | `GraphEdge` in `edges` list |
| `Checkpointer` | `StoringTransport` + `SqliteStore` |
| `graph.stream()` | `Runtime.stream(spec, prompt)` |

### Before (LangGraph)

```python
from langgraph.graph import StateGraph

builder = StateGraph(State)
builder.add_node("writer", writer_node)
builder.add_node("reviewer", reviewer_node)
builder.add_edge("writer", "reviewer")
graph = builder.compile(checkpointer=MemorySaver())
result = graph.invoke({"messages": [HumanMessage("write about agents")]})
```

### After (Ancora)

```python
from ancora import Runtime, AgentSpec, GraphSpec, GraphNode, GraphEdge

graph = GraphSpec(
    nodes=[
        GraphNode(id="writer", spec=AgentSpec("llama3", "Write about agents.")),
        GraphNode(id="reviewer", spec=AgentSpec("llama3", "Review the paragraph.")),
    ],
    edges=[GraphEdge(from_node="writer", to_node="reviewer")],
)
rt = Runtime()
result = rt.run_graph(graph, "write about agents")
```

## From CrewAI

| CrewAI concept | Ancora equivalent |
|----------------|-------------------|
| `Agent` | `AgentSpec` |
| `Task` | A run invocation `rt.run(spec, prompt)` |
| `Crew` | `GraphSpec` with nodes and edges |
| `Tool` | `@registry.tool(description=...)` |
| `Process.sequential` | Linear `GraphEdge` chain |
| `Process.hierarchical` | Fan-out `GraphEdge` from a manager node |

### Before (CrewAI)

```python
from crewai import Agent, Task, Crew

researcher = Agent(role="Researcher", goal="Find facts", llm=llm)
writer = Agent(role="Writer", goal="Write report", llm=llm)
task1 = Task(description="Research durable agents", agent=researcher)
task2 = Task(description="Write a report", agent=writer, context=[task1])
crew = Crew(agents=[researcher, writer], tasks=[task1, task2])
crew.kickoff()
```

### After (Ancora)

```python
graph = GraphSpec(
    nodes=[
        GraphNode(id="researcher", spec=AgentSpec("llama3", "Research durable agents.")),
        GraphNode(id="writer", spec=AgentSpec("llama3", "Write a report on the research.")),
    ],
    edges=[GraphEdge(from_node="researcher", to_node="writer")],
)
rt = Runtime()
result = rt.run_graph(graph, "durable agents")
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Durability](durability.md)
