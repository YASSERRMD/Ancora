"""Tests for Agent and Run async API."""

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
    return AgentSpec(name="test-agent", model_id="llama3", instructions="test")


async def test_agent_start_returns_run(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    assert isinstance(run, Run)


async def test_run_id_is_non_empty(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    run_id = run.run_id
    assert isinstance(run_id, str)
    assert len(run_id) > 0


async def test_single_agent_run_completes(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    kinds = [e for ev in events for e in [ev.decode("utf-8")]]
    assert any("started" in k for k in kinds)
    assert any("completed" in k for k in kinds)


async def test_async_event_iterator(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    collected = []
    async for ev in run:
        collected.append(ev)
    assert len(collected) >= 2


async def test_drain_events_returns_list(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert isinstance(events, list)
    assert all(isinstance(e, bytes) for e in events)


async def test_run_resume(rt):
    spec = AgentSpec(name="hil-agent", model_id="m", instructions="wait")
    agent = Agent(rt, spec)
    run = await agent.run()
    evs = []
    async for ev in run:
        evs.append(ev)
    await run.resume(b'{"choice": "yes"}')
    async for ev in run:
        evs.append(ev)
    event_strs = [e.decode("utf-8") for e in evs]
    assert any("resumed" in s for s in event_strs)


async def test_run_repr(rt, spec):
    agent = Agent(rt, spec)
    run = await agent.run()
    r = repr(run)
    assert "Run(id=" in r


def test_agent_spec_property(rt, spec):
    agent = Agent(rt, spec)
    assert agent.spec is spec


async def test_multiple_sequential_runs(rt, spec):
    agent = Agent(rt, spec)
    results = []
    for _ in range(3):
        run = await agent.run()
        evs = await run.drain_events()
        results.append(evs)
    assert len(results) == 3
    for evs in results:
        assert len(evs) >= 2


def test_agent_accessible_from_ancora_namespace(rt, spec):
    agent = ancora.Agent(rt, spec)
    assert isinstance(agent, Agent)
