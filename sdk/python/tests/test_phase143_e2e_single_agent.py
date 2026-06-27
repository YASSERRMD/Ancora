"""Phase 143 e2e task 1: single agent end to end."""

import json
import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_single_agent_run_id_non_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-sa", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert isinstance(run.run_id, str) and run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_drain_yields_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-drain", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_events_are_bytes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-bytes", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        assert isinstance(ev, bytes)
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_first_event_parseable():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-parse", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    first = json.loads(events[0])
    assert isinstance(first, dict)
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_second_run_distinct():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-twice", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run1 = await agent.run()
    run2 = await agent.run()
    assert run1.run_id != run2.run_id
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_stream_events_non_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    count = 0
    async for _ in run.stream_events():
        count += 1
    assert count > 0
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_repr_has_run_id():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-repr", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    assert run.run_id in repr(run)
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_with_system_prompt():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-sys", model_id="llama3", system_prompt="You are a helpful assistant.")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_runtime_not_freed_during_run():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-notfree", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert not rt.is_freed
    await run.drain_events()
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_repeated_drains_second_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="e2e-redrn", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    first = await run.drain_events()
    second = await run.drain_events()
    assert len(first) > 0
    assert len(second) == 0
    rt.free()
