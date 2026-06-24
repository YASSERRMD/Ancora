"""Human-in-the-loop example.

Demonstrates pausing an agent run and resuming it with a human decision.
Runs fully offline.

Usage::

    python -m examples.human_in_loop
"""

import asyncio
import json

import ancora


async def main() -> None:
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(
        name="hitl-agent",
        model_id="local-model",
        instructions="Ask for approval before taking action.",
    )
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    print(f"run {run.run_id}: draining initial events...")
    initial = await run.drain_events()
    for raw in initial:
        ev = json.loads(raw)
        print(f"  {ev['kind']}")

    print("human review: approving action...")
    await run.resume(b"approved")

    print("post-resume events:")
    resumed_events = await run.drain_events()
    for raw in resumed_events:
        ev = json.loads(raw)
        print(f"  {ev['kind']}")
        if ev["kind"] == "resumed":
            print(f"    decision: {ev.get('decision', '')}")

    rt.free()
    print("done.")


if __name__ == "__main__":
    asyncio.run(main())
