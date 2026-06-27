# Verifier and Consensus (Python)

Use a verifier agent to validate the primary agent's output before returning
it to the caller.

## Simple verifier

```python
from ancora import Runtime, AgentSpec, GraphSpec, GraphNode, GraphEdge

primary_spec = AgentSpec(
    model="llama3",
    instructions="Answer the user's question.",
)

verifier_spec = AgentSpec(
    model="llama3",
    instructions=(
        "Verify the previous agent's answer. "
        "If it is factually correct respond with 'VERIFIED'. "
        "Otherwise respond with 'REJECTED: <reason>'."
    ),
)

graph = GraphSpec(
    nodes=[
        GraphNode(id="primary", spec=primary_spec),
        GraphNode(id="verifier", spec=verifier_spec),
    ],
    edges=[GraphEdge(from_node="primary", to_node="verifier")],
)

rt = Runtime()
result = rt.run_graph(graph, "What is the capital of Egypt?")
print(result.output)
```

## N-verifier consensus

Run three verifiers and accept the answer only when the majority agree:

```python
import concurrent.futures

def run_verifier(index: int, candidate: str) -> bool:
    spec = AgentSpec(
        model="llama3",
        instructions="Is the following answer correct? Reply YES or NO.",
    )
    result = rt.run(spec, candidate)
    return result.output.strip().upper().startswith("YES")

candidate = rt.run(primary_spec, "What is the capital of Egypt?").output

with concurrent.futures.ThreadPoolExecutor() as pool:
    verdicts = list(pool.map(lambda i: run_verifier(i, candidate), range(3)))

if sum(verdicts) >= 2:
    print("ACCEPTED:", candidate)
else:
    print("REJECTED")
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Human-in-the-loop](human-in-the-loop.md)
