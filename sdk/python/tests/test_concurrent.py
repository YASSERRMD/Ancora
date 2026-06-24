"""Tests for concurrent and parallel agent runs."""

import asyncio
import pytest
import ancora
from ancora.agent import Agent
from ancora.models import AgentSpec


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


async def test_concurrent_agents(rt):
    spec1 = AgentSpec(name="agent-1", model_id="m")
    spec2 = AgentSpec(name="agent-2", model_id="m")

    run1, run2 = await asyncio.gather(
        Agent(rt, spec1).run(),
        Agent(rt, spec2).run(),
    )
    assert run1.run_id != run2.run_id

    evs1, evs2 = await asyncio.gather(
        run1.drain_events(),
        run2.drain_events(),
    )
    assert len(evs1) >= 2
    assert len(evs2) >= 2


async def test_parallel_event_drain(rt):
    agents = [Agent(rt, AgentSpec(name=f"a{i}", model_id="m")) for i in range(5)]
    runs = await asyncio.gather(*[ag.run() for ag in agents])
    all_events = await asyncio.gather(*[r.drain_events() for r in runs])
    for evs in all_events:
        assert len(evs) >= 2


async def test_run_ids_are_unique(rt):
    spec = AgentSpec(name="a", model_id="m")
    runs = await asyncio.gather(*[Agent(rt, spec).run() for _ in range(10)])
    ids = {r.run_id for r in runs}
    assert len(ids) == 10
