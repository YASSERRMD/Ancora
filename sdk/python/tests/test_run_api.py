"""Tests for Agent and Run async API."""

import asyncio
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


def run_async(coro):
    return asyncio.get_event_loop().run_until_complete(coro)


def test_agent_start_returns_run(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        run = await agent.run()
        return run

    run = run_async(go())
    assert isinstance(run, Run)


def test_run_id_is_non_empty(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        run = await agent.run()
        return run.run_id

    run_id = run_async(go())
    assert isinstance(run_id, str)
    assert len(run_id) > 0


def test_single_agent_run_completes(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        run = await agent.run()
        events = await run.drain_events()
        return events

    events = run_async(go())
    assert len(events) > 0
    kinds = [e for ev in events for e in [ev.decode("utf-8")]]
    assert any("started" in k for k in kinds)
    assert any("completed" in k for k in kinds)


def test_async_event_iterator(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        run = await agent.run()
        collected = []
        async for ev in run:
            collected.append(ev)
        return collected

    events = run_async(go())
    assert len(events) >= 2


def test_drain_events_returns_list(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        run = await agent.run()
        return await run.drain_events()

    events = run_async(go())
    assert isinstance(events, list)
    assert all(isinstance(e, bytes) for e in events)


def test_run_resume(rt):
    async def go():
        spec = AgentSpec(name="hil-agent", model_id="m", instructions="wait")
        agent = Agent(rt, spec)
        run = await agent.run()
        evs = []
        async for ev in run:
            evs.append(ev)
        await run.resume(b'{"choice": "yes"}')
        async for ev in run:
            evs.append(ev)
        return evs

    events = run_async(go())
    event_strs = [e.decode("utf-8") for e in events]
    assert any("resumed" in s for s in event_strs)


def test_run_repr(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        run = await agent.run()
        return repr(run)

    r = run_async(go())
    assert "Run(id=" in r


def test_agent_spec_property(rt, spec):
    agent = Agent(rt, spec)
    assert agent.spec is spec


def test_multiple_sequential_runs(rt, spec):
    async def go():
        agent = Agent(rt, spec)
        results = []
        for _ in range(3):
            run = await agent.run()
            evs = await run.drain_events()
            results.append(evs)
        return results

    all_runs = run_async(go())
    assert len(all_runs) == 3
    for evs in all_runs:
        assert len(evs) >= 2


def test_agent_accessible_from_ancora_namespace(rt, spec):
    agent = ancora.Agent(rt, spec)
    assert isinstance(agent, Agent)
