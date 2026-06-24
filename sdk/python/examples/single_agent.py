"""Single agent example.

Starts one agent run and prints the kind of each event. Runs fully offline.

Usage::

    python -m examples.single_agent
"""

import asyncio
import json

import ancora


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
        ev = json.loads(raw)
        kind = ev["kind"]
        if kind == "token":
            print(f"  token: {ev['text']!r}")
        else:
            print(f"  event: {kind}")

    rt.free()
    print("done.")


if __name__ == "__main__":
    asyncio.run(main())
