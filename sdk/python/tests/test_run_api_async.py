"""Async-native tests for Agent and Run using pytest-asyncio."""

import pytest
import ancora
from ancora.agent import Agent
from ancora.models import AgentSpec
from ancora.run import Run


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


@pytest.fixture
def spec():
    return AgentSpec(name="async-agent", model_id="llama3")


async def test_single_agent_run_async(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) >= 2


async def test_run_id_non_empty_async(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    assert run.run_id and isinstance(run.run_id, str)


async def test_events_have_kind_field(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        decoded = ev.decode("utf-8")
        assert "kind" in decoded


async def test_started_event_has_run_id(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    started = next(
        (e.decode("utf-8") for e in events if "started" in e.decode("utf-8")), None
    )
    assert started is not None
    assert run.run_id in started


async def test_completed_event_present(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    event_strs = [e.decode("utf-8") for e in events]
    assert any("completed" in s for s in event_strs)


async def test_resume_appends_resumed_event(rt):
    spec = AgentSpec(name="hil", model_id="m")
    agent = Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b'{"answer": "yes"}')
    events = await run.drain_events()
    event_strs = [e.decode("utf-8") for e in events]
    assert any("resumed" in s for s in event_strs)
    assert any("completed" in s for s in event_strs)


async def test_resumed_event_contains_decision(rt):
    spec = AgentSpec(name="hil2", model_id="m")
    agent = Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b"approved")
    events = await run.drain_events()
    event_strs = [e.decode("utf-8") for e in events]
    resumed = next((s for s in event_strs if "resumed" in s), None)
    assert resumed is not None
    assert "approved" in resumed


async def test_agent_run_is_a_run_instance(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    assert isinstance(run, Run)
