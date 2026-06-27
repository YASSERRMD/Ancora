"""Phase 142 task 8: human-in-loop suspend resume."""

import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_human_in_loop_run_can_resume():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b'{"decision":"approved"}')
    rt.free()


@pytest.mark.asyncio
async def test_human_in_loop_run_id_unchanged_after_resume():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-id", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    original_id = run.run_id
    await run.drain_events()
    await run.resume(b'{"decision":"approved"}')
    assert run.run_id == original_id
    rt.free()


@pytest.mark.asyncio
async def test_human_in_loop_resume_with_rejection():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-reject", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b'{"decision":"rejected","reason":"invalid"}')
    rt.free()


@pytest.mark.asyncio
async def test_human_in_loop_resume_with_empty_bytes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-empty", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b"")
    rt.free()


@pytest.mark.asyncio
async def test_human_in_loop_multiple_resumes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-multi", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    for i in range(3):
        await run.resume(f'{{"step":{i}}}'.encode())
        await run.drain_events()
    rt.free()


def test_human_in_loop_run_has_resume_method():
    from ancora.run import Run
    assert hasattr(Run, "resume")


@pytest.mark.asyncio
async def test_human_in_loop_events_after_resume_are_bytes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-bytes", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b'{"step":1}')
    events = await run.drain_events()
    for ev in events:
        assert isinstance(ev, bytes)
    rt.free()


@pytest.mark.asyncio
async def test_human_in_loop_resume_returns_none():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-ret", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    result = await run.resume(b'{"ok":true}')
    assert result is None
    rt.free()


@pytest.mark.asyncio
async def test_human_in_loop_two_runs_resume_independently():
    rt = ancora.Runtime()
    spec = AgentSpec(name="hil-two", model_id="llama3")
    agent = ancora.Agent(rt, spec)

    run1 = await agent.run()
    run2 = await agent.run()

    assert run1.run_id != run2.run_id

    await run1.drain_events()
    await run2.drain_events()

    await run1.resume(b'{"run":1}')
    await run2.resume(b'{"run":2}')
    rt.free()
