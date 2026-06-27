"""Phase 142 task 17: cancellation."""

import asyncio
import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_cancelled_task_raises_cancelled_error():
    rt = ancora.Runtime()
    spec = AgentSpec(name="cancel-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    async def drain_slow():
        await asyncio.sleep(0.1)
        await run.drain_events()

    task = asyncio.create_task(drain_slow())
    task.cancel()
    with pytest.raises(asyncio.CancelledError):
        await task
    rt.free()


@pytest.mark.asyncio
async def test_cancel_before_drain_does_not_corrupt_runtime():
    rt = ancora.Runtime()
    spec = AgentSpec(name="nocrash", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    task = asyncio.create_task(run.drain_events())
    task.cancel()
    try:
        await task
    except asyncio.CancelledError:
        pass

    run2 = await agent.run()
    assert run2.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_cancel_second_of_two_concurrent_tasks():
    rt = ancora.Runtime()
    spec = AgentSpec(name="dual-cancel", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run1, run2 = await asyncio.gather(agent.run(), agent.run())

    task1 = asyncio.create_task(run1.drain_events())
    task2 = asyncio.create_task(run2.drain_events())
    task2.cancel()

    events1 = await task1
    assert len(events1) > 0

    try:
        await task2
    except asyncio.CancelledError:
        pass

    rt.free()


@pytest.mark.asyncio
async def test_run_id_available_after_cancel():
    rt = ancora.Runtime()
    spec = AgentSpec(name="id-after-cancel", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    run_id = run.run_id

    task = asyncio.create_task(run.drain_events())
    task.cancel()
    try:
        await task
    except asyncio.CancelledError:
        pass

    assert run.run_id == run_id
    rt.free()


@pytest.mark.asyncio
async def test_runtime_usable_after_cancelled_run():
    rt = ancora.Runtime()
    spec = AgentSpec(name="post-cancel", model_id="llama3")
    agent = ancora.Agent(rt, spec)

    run1 = await agent.run()
    task = asyncio.create_task(run1.drain_events())
    task.cancel()
    try:
        await task
    except asyncio.CancelledError:
        pass

    run2 = await agent.run()
    events = await run2.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_timeout_wrapper_cancels_drain():
    rt = ancora.Runtime()
    spec = AgentSpec(name="timeout-drain", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    try:
        await asyncio.wait_for(run.drain_events(), timeout=0.001)
    except (asyncio.TimeoutError, asyncio.CancelledError):
        pass
    rt.free()


@pytest.mark.asyncio
async def test_free_runtime_is_idempotent_after_cancel():
    rt = ancora.Runtime()
    spec = AgentSpec(name="free-after-cancel", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    task = asyncio.create_task(run.drain_events())
    task.cancel()
    try:
        await task
    except asyncio.CancelledError:
        pass
    rt.free()
    rt.free()


@pytest.mark.asyncio
async def test_cancel_stream_events_generator():
    rt = ancora.Runtime()
    spec = AgentSpec(name="gen-cancel", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    async def consume_one():
        async for ev in run.stream_events():
            return ev

    task = asyncio.create_task(consume_one())
    await asyncio.sleep(0)
    task.cancel()
    try:
        await task
    except (asyncio.CancelledError, StopAsyncIteration):
        pass
    rt.free()
