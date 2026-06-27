"""Phase 142 task 3: single agent run async."""

import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_single_agent_run_returns_run_handle():
    rt = ancora.Runtime()
    spec = AgentSpec(name="sa-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run is not None
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_run_id_is_non_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="id-agent", model_id="gpt-4o")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_drain_events_returns_list():
    rt = ancora.Runtime()
    spec = AgentSpec(name="drain-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert isinstance(events, list)
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_drain_produces_at_least_one_event():
    rt = ancora.Runtime()
    spec = AgentSpec(name="evt-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_events_are_bytes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="bytes-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        assert isinstance(ev, bytes)
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_two_runs_have_different_ids():
    rt = ancora.Runtime()
    spec = AgentSpec(name="two-runs", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run1 = await agent.run()
    run2 = await agent.run()
    assert run1.run_id != run2.run_id
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_async_for_yields_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="afor-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = []
    async for ev in run:
        events.append(ev)
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_drain_then_drain_is_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="drain2-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    ev1 = await run.drain_events()
    ev2 = await run.drain_events()
    assert len(ev2) == 0
    _ = ev1
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_stream_events_is_async_gen():
    rt = ancora.Runtime()
    spec = AgentSpec(name="stream-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = []
    async for ev in run.stream_events():
        events.append(ev)
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_single_agent_spec_is_accessible():
    rt = ancora.Runtime()
    spec = AgentSpec(name="acc-agent", model_id="llama3", instructions="hello")
    agent = ancora.Agent(rt, spec)
    assert agent.spec.name == "acc-agent"
    assert agent.spec.instructions == "hello"
    rt.free()
