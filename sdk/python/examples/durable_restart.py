"""Durable restart example.

Demonstrates persisting run state and events to a MemoryStore so that a
restarted process can replay the journal and recover where it left off
without re-running the agent.
Runs fully offline.

Usage::

    python -m examples.durable_restart
"""

from __future__ import annotations

import asyncio
import json
from typing import Any

import ancora
from ancora import MemoryStore


class RunJournal:
    """Simple in-process event journal backed by a MemoryStore.

    Mirrors the role of a real durable store (SQLite, Redis, etc.) for
    offline demonstration purposes.
    """

    def __init__(self, mem: MemoryStore) -> None:
        self._mem = mem

    def record_run(self, run_id: str) -> None:
        runs = self._mem.read("runs", default=[])
        if run_id not in runs:
            runs.append(run_id)
        self._mem.write("runs", runs)
        self._mem.write(f"events:{run_id}", [])

    def append_event(self, run_id: str, payload: bytes) -> None:
        key = f"events:{run_id}"
        events = self._mem.read(key, default=[])
        events.append(payload.decode("utf-8", errors="replace"))
        self._mem.write(key, events)

    def events_for_run(self, run_id: str) -> list[str]:
        return self._mem.read(f"events:{run_id}", default=[])

    def run_count(self) -> int:
        return len(self._mem.read("runs", default=[]))


async def main() -> None:
    mem = MemoryStore()
    journal = RunJournal(mem)

    rt = ancora.Runtime()
    spec = ancora.AgentSpec(
        name="durable-agent",
        model_id="local-model",
        instructions="Respond and let your events be persisted.",
    )
    agent = ancora.Agent(rt, spec)

    # --- first run ---
    run = await agent.run()
    run_id = run.run_id
    journal.record_run(run_id)
    print(f"first run: {run_id}")

    async for ev in run:
        journal.append_event(run_id, ev)

    events = journal.events_for_run(run_id)
    print(f"live events persisted: {len(events)}")

    # --- simulate restart: replay from journal ---
    replayed = journal.events_for_run(run_id)
    print(f"replayed {len(replayed)} event(s) from journal")

    total = journal.run_count()
    print(f"total runs in journal: {total}")

    rt.free()
    print("durable-restart done.")


if __name__ == "__main__":
    asyncio.run(main())
