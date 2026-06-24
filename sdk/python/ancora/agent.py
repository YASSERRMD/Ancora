"""Pythonic Agent class wrapping the Ancora runtime transport."""

from __future__ import annotations

import asyncio
from typing import TYPE_CHECKING

from ancora.models import AgentSpec
from ancora.run import Run
from ancora.wire import to_wire_bytes

if TYPE_CHECKING:
    from ancora._ancora import Runtime


class Agent:
    """Starts agent runs via the local runtime.

    Example::

        import asyncio
        import ancora

        async def main():
            rt = ancora.Runtime()
            spec = ancora.AgentSpec(name="a", model_id="llama3")
            agent = ancora.Agent(rt, spec)
            run = await agent.run()
            events = await run.drain_events()
            rt.free()

        asyncio.run(main())
    """

    def __init__(self, rt: "Runtime", spec: AgentSpec) -> None:
        self._rt = rt
        self._spec = spec

    @property
    def spec(self) -> AgentSpec:
        """Return the agent specification."""
        return self._spec

    async def run(self) -> Run:
        """Start a new run and return a :class:`Run` handle."""
        await asyncio.sleep(0)
        wire = to_wire_bytes(self._spec)
        run_id = self._rt.start_run(wire)
        return Run(self._rt, run_id)
