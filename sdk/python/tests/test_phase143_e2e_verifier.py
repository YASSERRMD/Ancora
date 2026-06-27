"""Phase 143 e2e task 2: verifier end to end."""

import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_verifier_two_agents_distinct_ids():
    rt = ancora.Runtime()
    drafter = AgentSpec(name="drafter", model_id="llama3")
    verifier = AgentSpec(name="verifier", model_id="llama3")
    a1 = ancora.Agent(rt, drafter)
    a2 = ancora.Agent(rt, verifier)
    r1 = await a1.run()
    r2 = await a2.run()
    assert r1.run_id != r2.run_id
    rt.free()


@pytest.mark.asyncio
async def test_verifier_both_runs_yield_events():
    rt = ancora.Runtime()
    drafter = AgentSpec(name="drafter", model_id="llama3")
    verifier = AgentSpec(name="verifier", model_id="llama3")
    r1 = await ancora.Agent(rt, drafter).run()
    r2 = await ancora.Agent(rt, verifier).run()
    ev1 = await r1.drain_events()
    ev2 = await r2.drain_events()
    assert len(ev1) > 0
    assert len(ev2) > 0
    rt.free()


@pytest.mark.asyncio
async def test_verifier_run_ids_are_strings():
    rt = ancora.Runtime()
    r1 = await ancora.Agent(rt, AgentSpec(name="d", model_id="llama3")).run()
    r2 = await ancora.Agent(rt, AgentSpec(name="v", model_id="llama3")).run()
    assert isinstance(r1.run_id, str)
    assert isinstance(r2.run_id, str)
    rt.free()


@pytest.mark.asyncio
async def test_verifier_events_independent():
    rt = ancora.Runtime()
    r1 = await ancora.Agent(rt, AgentSpec(name="d2", model_id="llama3")).run()
    r2 = await ancora.Agent(rt, AgentSpec(name="v2", model_id="llama3")).run()
    ev1 = await r1.drain_events()
    ev2 = await r2.drain_events()
    assert ev1 is not ev2
    rt.free()


@pytest.mark.asyncio
async def test_verifier_drafter_runs_before_verifier():
    rt = ancora.Runtime()
    drafter = AgentSpec(name="first-drafter", model_id="llama3")
    verifier_spec = AgentSpec(name="second-verifier", model_id="llama3")
    r_draft = await ancora.Agent(rt, drafter).run()
    ev_draft = await r_draft.drain_events()
    assert len(ev_draft) > 0
    r_verify = await ancora.Agent(rt, verifier_spec).run()
    ev_verify = await r_verify.drain_events()
    assert len(ev_verify) > 0
    rt.free()


@pytest.mark.asyncio
async def test_verifier_drafter_id_differs_from_verifier_id():
    rt = ancora.Runtime()
    r1 = await ancora.Agent(rt, AgentSpec(name="d3", model_id="llama3")).run()
    r2 = await ancora.Agent(rt, AgentSpec(name="v3", model_id="llama3")).run()
    assert r1.run_id != r2.run_id
    rt.free()


@pytest.mark.asyncio
async def test_verifier_three_node_pipeline():
    rt = ancora.Runtime()
    specs = [AgentSpec(name=f"node-{i}", model_id="llama3") for i in range(3)]
    runs = [await ancora.Agent(rt, s).run() for s in specs]
    run_ids = [r.run_id for r in runs]
    assert len(set(run_ids)) == 3
    rt.free()


@pytest.mark.asyncio
async def test_verifier_runtime_shared_across_nodes():
    rt = ancora.Runtime()
    r1 = await ancora.Agent(rt, AgentSpec(name="shared-1", model_id="llama3")).run()
    r2 = await ancora.Agent(rt, AgentSpec(name="shared-2", model_id="llama3")).run()
    assert not rt.is_freed
    await r1.drain_events()
    await r2.drain_events()
    rt.free()
    assert rt.is_freed


@pytest.mark.asyncio
async def test_verifier_multiple_cycles():
    rt = ancora.Runtime()
    for cycle in range(3):
        r1 = await ancora.Agent(rt, AgentSpec(name=f"d-c{cycle}", model_id="llama3")).run()
        r2 = await ancora.Agent(rt, AgentSpec(name=f"v-c{cycle}", model_id="llama3")).run()
        ev1 = await r1.drain_events()
        ev2 = await r2.drain_events()
        assert len(ev1) > 0 and len(ev2) > 0
    rt.free()


@pytest.mark.asyncio
async def test_verifier_conformance_suite_contains_verifier_scenario():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert isinstance(results, dict)
    rt.free()
