"""Tool composition example.

Demonstrates tools that invoke other tools (composition pattern) via a shared
ToolRegistry. Runs fully offline.

Usage::

    python -m examples.tool_composition
"""

import asyncio
import json

import ancora
from ancora.tools import ToolRegistry, tool

registry = ToolRegistry()


@tool
def tokenize(text: str) -> list:
    """Split text into whitespace-separated tokens."""
    return text.split()


@tool
def count_tokens(text: str) -> int:
    """Count tokens in text by calling the tokenize tool."""
    tokens = registry.dispatch("tokenize", json.dumps({"text": text}))
    return len(tokens)


@tool
def word_frequencies(text: str) -> dict:
    """Return a word frequency map for text."""
    tokens = registry.dispatch("tokenize", json.dumps({"text": text}))
    freq: dict[str, int] = {}
    for t in tokens:
        freq[t] = freq.get(t, 0) + 1
    return freq


for t in [tokenize, count_tokens, word_frequencies]:
    registry.register(t)


async def main() -> None:
    text = "the quick brown fox jumps over the lazy dog the fox"

    tokens = registry.dispatch("tokenize", json.dumps({"text": text}))
    print(f"tokens ({len(tokens)}): {tokens}")

    count = registry.dispatch("count_tokens", json.dumps({"text": text}))
    print(f"count_tokens: {count}")

    freqs = registry.dispatch("word_frequencies", json.dumps({"text": text}))
    print(f"word_frequencies: {freqs}")

    rt = ancora.Runtime()
    spec = ancora.AgentSpec(
        name="composer",
        model_id="local-model",
        tools=registry.all_specs(),
    )
    agent = ancora.Agent(rt, spec, registry=registry)
    run = await agent.run()
    await run.drain_events()
    print(f"agent has {len(agent.spec.tools)} tools registered")

    rt.free()


if __name__ == "__main__":
    asyncio.run(main())
