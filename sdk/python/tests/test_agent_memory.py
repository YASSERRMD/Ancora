"""Tests for Agent memory integration."""

import ancora
from ancora.memory import MemoryStore


async def test_agent_default_memory_is_empty():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    assert isinstance(agent.memory, MemoryStore)
    assert len(agent.memory) == 0
    rt.free()


async def test_agent_accepts_provided_memory():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    mem = MemoryStore()
    mem.write("user", "Bob")
    agent = ancora.Agent(rt, spec, memory=mem)
    assert agent.memory.read("user") == "Bob"
    rt.free()


async def test_agent_memory_persists_across_runs():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)

    agent.memory.write("count", 1)
    run1 = await agent.run()
    await run1.drain_events()

    assert agent.memory.read("count") == 1

    agent.memory.write("count", 2)
    run2 = await agent.run()
    await run2.drain_events()

    assert agent.memory.read("count") == 2
    rt.free()


async def test_agent_memory_shared_with_caller():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    mem = MemoryStore()
    agent = ancora.Agent(rt, spec, memory=mem)

    agent.memory.write("x", 99)
    assert mem.read("x") == 99
    rt.free()


async def test_agent_memory_via_ancora_namespace():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    mem = ancora.MemoryStore()
    mem.write("greeting", "hello")
    agent = ancora.Agent(rt, spec, memory=mem)
    assert agent.memory.read("greeting") == "hello"
    rt.free()
