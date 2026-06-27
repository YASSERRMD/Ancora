# Memory and RAG (Python)

Inject retrieved context into an agent's instructions to ground responses
in your own data.

## Offline keyword retrieval (no vector database needed)

```python
import re

def keyword_retrieve(corpus: list[dict], query: str, top_k: int = 3) -> list[dict]:
    terms = set(re.findall(r'\w+', query.lower()))
    scored = []
    for passage in corpus:
        text_terms = set(re.findall(r'\w+', passage["content"].lower()))
        score = len(terms & text_terms)
        if score > 0:
            scored.append((score, passage))
    scored.sort(key=lambda x: x[0], reverse=True)
    return [p for _, p in scored[:top_k]]

corpus = [
    {"key": "doc1", "content": "Ancora provides durable agent orchestration."},
    {"key": "doc2", "content": "LanceDB is an embedded vector database for Python."},
    {"key": "doc3", "content": "Pydantic validates structured data in Python."},
]

hits = keyword_retrieve(corpus, "durable agent")
context = "\n".join(h["content"] for h in hits)
```

## Inject context into the agent

```python
from ancora import Runtime, AgentSpec

rt = Runtime()

spec = AgentSpec(
    model="llama3",
    instructions=f"Answer using only the following context:\n\n{context}",
)

result = rt.run(spec, "What does Ancora provide?")
print(result.output)
```

## LanceDB vector retrieval

```python
import lancedb
import numpy as np

db = lancedb.connect("/var/lib/myapp/vectors")
table = db.open_table("passages")

query_vector = embed("durable agent")   # your embedding function
results = table.search(query_vector).limit(3).to_pandas()
context = "\n".join(results["content"].tolist())
```

## Inject context as a tool

```python
@registry.tool(description="Retrieve relevant passages for the query.")
def retrieve(query: str) -> str:
    hits = keyword_retrieve(corpus, query)
    return "\n".join(h["content"] for h in hits)

spec = AgentSpec(model="llama3", instructions="Use retrieve to answer questions.", tools=registry)
```

## See also

- [Vector stores](vector-stores.md)
- [Providers](providers.md)
