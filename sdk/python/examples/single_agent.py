"""Single agent example.

Starts one agent run and prints the kind of each event. Runs fully offline.

Usage::

    python -m examples.single_agent
"""

import asyncio

import ancora
from examples.helpers import print_event


async def main() -> None:
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(
        name="demo-agent",
        model_id="local-model",
        instructions="Respond to the user.",
    )
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    print(f"started run: {run.run_id}")

    async for raw in run.stream_events():
        print_event(raw)

    rt.free()
    print("done.")


if __name__ == "__main__":
    asyncio.run(main())
