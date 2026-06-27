"""Qwen via gateway example.

Demonstrates configuring an AgentSpec for the Qwen model family (Alibaba
DashScope-compatible) and running it through the standard agent transport.
The model name is resolved by the runtime to the configured provider endpoint.
Runs fully offline -- no DashScope key required.

Usage::

    python -m examples.qwen_gateway
"""

from __future__ import annotations

import asyncio

import ancora
from examples.helpers import print_event

# Well-known Qwen model identifiers.
QWEN_MODELS = [
    "qwen-turbo",
    "qwen-plus",
    "qwen-max",
    "qwen-long",
]


async def run_qwen_agent(rt: ancora.Runtime, model_id: str) -> None:
    spec = ancora.AgentSpec(
        name=f"qwen-{model_id}-agent",
        model_id=model_id,
        instructions="You are a helpful assistant powered by Qwen.",
    )
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    print(f"model={model_id}  run_id={run.run_id}")
    events = await run.drain_events()
    print(f"  events: {len(events)}")


async def main() -> None:
    rt = ancora.Runtime()
    for model in QWEN_MODELS:
        await run_qwen_agent(rt, model)
    rt.free()
    print("qwen-gateway done.")


if __name__ == "__main__":
    asyncio.run(main())
