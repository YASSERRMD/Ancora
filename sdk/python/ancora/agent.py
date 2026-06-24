"""Pythonic Agent class wrapping the Ancora runtime transport."""

from __future__ import annotations

import asyncio
from typing import TYPE_CHECKING, Optional

from ancora.models import AgentSpec
from ancora.run import Run
from ancora.wire import to_wire_bytes

if TYPE_CHECKING:
    from ancora._ancora import Runtime
    from ancora.tools import Tool, ToolRegistry


class Agent:
    """Starts agent runs via the local runtime.

    Optionally accepts a :class:`~ancora.tools.ToolRegistry` for callback
    dispatch when tool-call events arrive.

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

    def __init__(
        self,
        rt: "Runtime",
        spec: AgentSpec,
        registry: "Optional[ToolRegistry]" = None,
    ) -> None:
        self._rt = rt
        self._spec = spec
        self._registry = registry

    @property
    def spec(self) -> AgentSpec:
        """Return the agent specification."""
        return self._spec

    @property
    def registry(self) -> "Optional[ToolRegistry]":
        """Return the tool registry, if any."""
        return self._registry

    def register_tool(self, t: "Tool") -> None:
        """Register a tool with this agent's registry.

        Creates a new :class:`~ancora.tools.ToolRegistry` if one hasn't
        been set yet.
        """
        from ancora.tools import ToolRegistry
        if self._registry is None:
            self._registry = ToolRegistry()
        self._registry.register(t)
        tools = list(self._spec.tools) + [t.spec]
        self._spec = self._spec.model_copy(update={"tools": tools})

    async def run(self) -> Run:
        """Start a new run and return a :class:`Run` handle."""
        await asyncio.sleep(0)
        wire = to_wire_bytes(self._spec)
        run_id = self._rt.start_run(wire)
        return Run(self._rt, run_id)
