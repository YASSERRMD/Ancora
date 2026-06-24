"""Edge case tests for runtime and run behavior."""

import pytest
import ancora
from ancora.agent import Agent
from ancora.models import AgentSpec


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


def test_poll_run_unknown_id_returns_none(rt):
    ev = rt.poll_run("nonexistent-run-id")
    assert ev is None


def test_resume_unknown_run_id_is_noop(rt):
    rt.resume_run("nonexistent-run-id", b"ok")


async def test_concurrent_runs_two_runtimes():
    import asyncio

    rt1 = ancora.Runtime()
    rt2 = ancora.Runtime()
    try:
        spec = AgentSpec(name="a", model_id="m")

        run1, run2 = await asyncio.gather(
            Agent(rt1, spec).run(),
            Agent(rt2, spec).run(),
        )
        evs1, evs2 = await asyncio.gather(
            run1.drain_events(),
            run2.drain_events(),
        )
        assert len(evs1) >= 2
        assert len(evs2) >= 2
        assert run1.run_id != run2.run_id
    finally:
        rt1.free()
        rt2.free()


async def test_drain_empty_run(rt):
    spec = AgentSpec(name="a", model_id="m")
    agent = Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    leftover = await run.drain_events()
    assert leftover == []
