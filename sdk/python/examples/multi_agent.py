"""Multi-agent example.

Demonstrates running two agents concurrently and collecting their results.
Runs fully offline.

Usage::

    python -m examples.multi_agent
"""

import asyncio
import json

import ancora


async def run_agent(rt, name: str, model_id: str) -> list[str]:
    spec = ancora.AgentSpec(name=name, model_id=model_id)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    tokens = [t async for t in run.stream_tokens()]
    return tokens


async def main() -> None:
    rt = ancora.Runtime()

    researcher, writer = await asyncio.gather(
        run_agent(rt, "researcher", "local-model"),
        run_agent(rt, "writer", "local-model"),
    )

    print(f"researcher output: {''.join(researcher)!r}")
    print(f"writer output:     {''.join(writer)!r}")

    rt.free()
    print("both agents completed.")


if __name__ == "__main__":
    asyncio.run(main())
