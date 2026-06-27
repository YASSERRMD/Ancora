"""Phase 142 task 16: concurrent runs isolation."""

import asyncio
import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_two_concurrent_runs_have_distinct_ids():
    rt = ancora.Runtime()
    spec = AgentSpec(name="conc-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run1, run2 = await asyncio.gather(agent.run(), agent.run())
    assert run1.run_id != run2.run_id
    rt.free()


@pytest.mark.asyncio
async def test_five_concurrent_runs_all_unique():
    rt = ancora.Runtime()
    spec = AgentSpec(name="fan-out", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = await asyncio.gather(*[agent.run() for _ in range(5)])
    ids = [r.run_id for r in runs]
    assert len(set(ids)) == 5
    rt.free()


@pytest.mark.asyncio
async def test_concurrent_runs_do_not_share_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="isolated", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run1, run2 = await asyncio.gather(agent.run(), agent.run())
    ev1 = await run1.drain_events()
    ev2 = await run2.drain_events()
    assert len(ev1) > 0
    assert len(ev2) > 0
    assert ev1 is not ev2
    rt.free()


@pytest.mark.asyncio
async def test_concurrent_runtimes_do_not_share_state():
    rt1 = ancora.Runtime()
    rt2 = ancora.Runtime()
    spec = AgentSpec(name="rt-iso", model_id="llama3")
    agent1 = ancora.Agent(rt1, spec)
    agent2 = ancora.Agent(rt2, spec)
    run1, run2 = await asyncio.gather(agent1.run(), agent2.run())
    assert run1.run_id != run2.run_id
    rt1.free()
    rt2.free()


@pytest.mark.asyncio
async def test_ten_concurrent_runs_complete():
    rt = ancora.Runtime()
    spec = AgentSpec(name="bulk", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = await asyncio.gather(*[agent.run() for _ in range(10)])
    assert all(r.run_id != "" for r in runs)
    rt.free()


@pytest.mark.asyncio
async def test_concurrent_drain_does_not_raise():
    rt = ancora.Runtime()
    spec = AgentSpec(name="drain-conc", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run1, run2 = await asyncio.gather(agent.run(), agent.run())
    results = await asyncio.gather(run1.drain_events(), run2.drain_events())
    assert all(isinstance(r, list) for r in results)
    rt.free()


@pytest.mark.asyncio
async def test_concurrent_runs_run_id_is_non_empty_string():
    rt = ancora.Runtime()
    spec = AgentSpec(name="run-id-check", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = await asyncio.gather(*[agent.run() for _ in range(3)])
    for r in runs:
        assert isinstance(r.run_id, str) and r.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_concurrent_agents_different_models():
    rt = ancora.Runtime()
    specs = [
        AgentSpec(name="a1", model_id="llama3"),
        AgentSpec(name="a2", model_id="gpt-4o"),
        AgentSpec(name="a3", model_id="claude-opus-4-8"),
    ]
    agents = [ancora.Agent(rt, s) for s in specs]
    runs = await asyncio.gather(*[a.run() for a in agents])
    ids = {r.run_id for r in runs}
    assert len(ids) == 3
    rt.free()
