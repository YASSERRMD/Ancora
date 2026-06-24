"""Streaming tokens example.

Demonstrates streaming individual tokens from a run as they arrive.
Runs fully offline.

Usage::

    python -m examples.streaming
"""

import asyncio

import ancora


async def main() -> None:
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="streamer", model_id="local-model")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    print(f"run {run.run_id} - streaming tokens:")
    print("  ", end="", flush=True)
    async for token in run.stream_tokens():
        print(token, end="", flush=True)
    print()
    print("stream complete.")

    rt.free()


if __name__ == "__main__":
    asyncio.run(main())
