"""Async tools example.

Demonstrates registering tools with async callbacks and dispatching them
via ToolRegistry.adispatch. Runs fully offline.

Usage::

    python -m examples.async_tools
"""

import asyncio

import ancora
from ancora.tools import ToolRegistry, tool


@tool
def sync_fetch(url: str) -> str:
    """Fetch a URL synchronously (simulated)."""
    return f"<html>content of {url}</html>"


@tool
def async_process(data: str) -> str:
    """Process data asynchronously (returns a coroutine)."""
    async def _inner():
        await asyncio.sleep(0)
        return data.upper()
    return _inner()


async def main() -> None:
    registry = ToolRegistry()
    registry.register(sync_fetch)
    registry.register(async_process)

    result1 = await registry.adispatch("sync_fetch", '{"url": "http://example.com"}')
    print(f"sync_fetch: {result1}")

    result2 = await registry.adispatch("async_process", '{"data": "hello"}')
    print(f"async_process: {result2}")

    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="async-tool-agent", model_id="local-model")
    agent = ancora.Agent(rt, spec, registry=registry)
    run = await agent.run()
    await run.drain_events()
    rt.free()
    print("done.")


if __name__ == "__main__":
    asyncio.run(main())
