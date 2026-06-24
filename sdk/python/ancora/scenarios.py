"""Built-in conformance scenarios for the Ancora Python SDK.

Import and call :func:`register_builtin_scenarios` to add all standard
scenarios to a :class:`~ancora.conformance.ConformanceSuite`.
"""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

import ancora
from ancora.conformance import ConformanceSuite
from ancora.memory import MemoryStore
from ancora.models import AgentSpec, StreamEvent
from ancora.tools import ToolRegistry, tool
from ancora.wire import from_wire_bytes, to_wire_bytes

if TYPE_CHECKING:
    pass


async def _single_run(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    if not events:
        return False
    kinds = [json.loads(e)["kind"] for e in events]
    return kinds[0] == "started" and kinds[-1] == "completed"


async def _human_in_loop(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b"approved")
    events = await run.drain_events()
    if not events:
        return False
    kinds = [json.loads(e)["kind"] for e in events]
    return "resumed" in kinds and kinds[-1] == "completed"


async def _multi_run_isolation(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run1 = await agent.run()
    run2 = await agent.run()
    return run1.run_id != run2.run_id


async def _spec_roundtrip(rt) -> bool:
    spec = AgentSpec(name="conformance-agent", model_id="test-model")
    wire = to_wire_bytes(spec)
    recovered = from_wire_bytes(wire)
    return recovered.name == spec.name and recovered.model_id == spec.model_id


async def _tool_wire_format(rt) -> bool:
    @tool
    def add(a: int, b: int) -> int:
        """Add two integers."""
        return a + b

    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    agent.register_tool(add)
    wire = to_wire_bytes(agent.spec)
    recovered = from_wire_bytes(wire)
    return len(recovered.tools) == 1 and recovered.tools[0].name == "add"


async def _streaming_tokens(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    tokens = [t async for t in run.stream_tokens()]
    return len(tokens) > 0 and all(isinstance(t, str) for t in tokens)


async def _memory_persistence(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    mem = MemoryStore()
    agent = ancora.Agent(rt, spec, memory=mem)
    agent.memory.write("key", "value")
    run = await agent.run()
    await run.drain_events()
    return agent.memory.read("key") == "value"


async def _event_count(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    return len(events) == 5


async def _stream_event_types(rt) -> bool:
    spec = AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    parsed = [StreamEvent.from_bytes(e) async for e in run.stream_events()]
    token_count = sum(1 for e in parsed if e.is_token)
    started_count = sum(1 for e in parsed if e.is_started)
    completed_count = sum(1 for e in parsed if e.is_completed)
    return token_count == 3 and started_count == 1 and completed_count == 1


def register_builtin_scenarios(suite: ConformanceSuite) -> None:
    """Register all standard scenarios onto *suite*."""
    suite.register("single_run", _single_run)
    suite.register("human_in_loop", _human_in_loop)
    suite.register("multi_run_isolation", _multi_run_isolation)
    suite.register("spec_roundtrip", _spec_roundtrip)
    suite.register("tool_wire_format", _tool_wire_format)
    suite.register("streaming_tokens", _streaming_tokens)
    suite.register("memory_persistence", _memory_persistence)
    suite.register("event_count", _event_count)
    suite.register("stream_event_types", _stream_event_types)
