"""Tests for decorated tools running in agent runs."""

import json
import pytest
import ancora
from ancora.agent import Agent
from ancora.models import AgentSpec, EffectClass
from ancora.tools import Tool, ToolRegistry, tool


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


def test_decorated_tool_spec_in_agent():
    @tool
    def search(query: str) -> str:
        """Search the web."""
        return f"results: {query}"

    spec = AgentSpec(name="agent", model_id="m", tools=[search.spec])
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "search"


async def test_agent_register_tool_adds_to_spec(rt):
    @tool
    def calc(n: int) -> int:
        return n * 2

    spec = AgentSpec(name="agent", model_id="m")
    agent = Agent(rt, spec)
    agent.register_tool(calc)

    assert len(agent.spec.tools) == 1
    assert agent.spec.tools[0].name == "calc"


async def test_agent_register_multiple_tools(rt):
    @tool
    def t1() -> None: pass
    @tool
    def t2() -> None: pass
    @tool
    def t3() -> None: pass

    spec = AgentSpec(name="agent", model_id="m")
    agent = Agent(rt, spec)
    agent.register_tool(t1)
    agent.register_tool(t2)
    agent.register_tool(t3)

    assert len(agent.spec.tools) == 3


async def test_decorated_tool_runs_in_run(rt):
    @tool(effect_class=EffectClass.PURE)
    def double(n: int) -> int:
        """Double a number."""
        return n * 2

    registry = ToolRegistry()
    registry.register(double)

    spec = AgentSpec(name="agent", model_id="m", tools=[double.spec])
    agent = Agent(rt, spec, registry=registry)

    run = await agent.run()
    events = await run.drain_events()
    assert len(events) >= 2
    started = json.loads(bytes(events[0]).decode("utf-8"))
    assert started["kind"] == "started"


async def test_agent_tool_spec_in_wire_bytes(rt):
    @tool
    def fetch(url: str) -> str:
        return url

    spec = AgentSpec(name="agent", model_id="m", tools=[fetch.spec])
    wire = spec.wire_bytes()
    parsed = json.loads(wire)
    assert len(parsed["tools"]) == 1
    assert parsed["tools"][0]["name"] == "fetch"


async def test_tool_registry_dispatch_in_agent(rt):
    results = []

    @tool
    def append_item(item: str) -> None:
        results.append(item)

    reg = ToolRegistry()
    reg.register(append_item)

    spec = AgentSpec(name="agent", model_id="m")
    agent = Agent(rt, spec, registry=reg)
    run = await agent.run()
    await run.drain_events()

    reg.dispatch("append_item", '{"item": "hello"}')
    assert "hello" in results
