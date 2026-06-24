"""RAG with memory example.

Demonstrates a retrieval-augmented agent that stores retrieved context in a
MemoryStore and reads it back. Runs fully offline.

Usage::

    python -m examples.rag_memory
"""

import asyncio
import json
from typing import Optional

import ancora
from ancora import MemoryStore
from ancora.tools import ToolRegistry, tool


@tool(name="retrieve", description="Retrieve documents for a query.")
def retrieve(query: str, top_k: int = 3) -> str:
    """Return a fake retrieval result for offline testing."""
    return f"[doc1: {query} overview] [doc2: {query} details] [doc3: related]"


@tool(name="summarize", description="Summarize a block of text.")
def summarize(text: str) -> str:
    """Return a fake summary for offline testing."""
    words = text.split()[:10]
    return " ".join(words) + " ..."


async def main() -> None:
    rt = ancora.Runtime()
    mem = MemoryStore()

    registry = ToolRegistry()
    registry.register(retrieve)
    registry.register(summarize)

    spec = ancora.AgentSpec(
        name="rag-agent",
        model_id="local-model",
        tools=registry.all_specs(),
    )
    agent = ancora.Agent(rt, spec, registry=registry, memory=mem)

    query = "Ancora agent runtime architecture"
    docs = registry.dispatch("retrieve", json.dumps({"query": query, "top_k": 2}))
    agent.memory.write("retrieved_docs", docs)
    print(f"retrieved: {docs}")

    summary = registry.dispatch("summarize", json.dumps({"text": docs}))
    agent.memory.write("summary", summary)
    print(f"summary: {summary}")

    run = await agent.run()
    tokens = [t async for t in run.stream_tokens()]
    agent.memory.write("response", "".join(tokens))
    print(f"response: {agent.memory.read('response')}")
    print(f"memory keys: {agent.memory.keys}")

    rt.free()


if __name__ == "__main__":
    asyncio.run(main())
