"""Phase 143 e2e task 3: human-in-loop end to end."""

import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_hil_run_suspends_and_resumes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-agent", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    await run.approve()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_hil_run_can_be_rejected():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-reject", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.reject("Policy violation")
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_hil_approve_unblocks_streaming():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-stream", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.approve()
    count = 0
    async for _ in run.stream_events():
        count += 1
    assert count > 0
    rt.free()


@pytest.mark.asyncio
async def test_hil_run_id_persists_through_suspend():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-persist", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    run_id = run.run_id
    await run.approve()
    await run.drain_events()
    assert run.run_id == run_id
    rt.free()


@pytest.mark.asyncio
async def test_hil_multiple_approval_checkpoints():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-multi", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    for _ in range(3):
        run = await agent.run()
        await run.approve()
        events = await run.drain_events()
        assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_hil_rejection_reason_accepted():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-reason", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.reject(reason="Blocked by compliance policy")
    await run.drain_events()
    rt.free()


@pytest.mark.asyncio
async def test_hil_no_approval_required_runs_immediately():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-free", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_hil_approve_twice_is_idempotent():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-idem", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.approve()
    await run.approve()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_hil_repr_contains_run_id():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-repr", model_id="llama3", require_approval=True)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id in repr(run)
    await run.approve()
    await run.drain_events()
    rt.free()
