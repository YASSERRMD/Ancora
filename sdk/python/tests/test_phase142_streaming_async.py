"""Phase 142 task 9: streaming async generator."""

import json
import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_streaming_stream_events_yields_bytes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="stream-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = []
    async for ev in run.stream_events():
        events.append(ev)
        assert isinstance(ev, bytes)
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_stream_tokens_yields_strings():
    rt = ancora.Runtime()
    spec = AgentSpec(name="token-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    tokens = []
    async for tok in run.stream_tokens():
        tokens.append(tok)
        assert isinstance(tok, str)
    rt.free()


@pytest.mark.asyncio
async def test_streaming_first_event_is_parseable_json():
    rt = ancora.Runtime()
    spec = AgentSpec(name="json-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    first = None
    async for ev in run.stream_events():
        first = ev
        break
    if first is not None:
        obj = json.loads(first)
        assert "kind" in obj or len(obj) > 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_drain_events_collects_all():
    rt = ancora.Runtime()
    spec = AgentSpec(name="drain-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_events_are_non_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="nonempty-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    async for ev in run.stream_events():
        assert len(ev) > 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_async_for_protocol():
    rt = ancora.Runtime()
    spec = AgentSpec(name="aiter-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    count = 0
    async for ev in run:
        count += 1
        assert isinstance(ev, bytes)
    assert count > 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_two_runs_do_not_share_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="two-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)

    run1 = await agent.run()
    run2 = await agent.run()

    assert run1.run_id != run2.run_id

    ev1 = await run1.drain_events()
    ev2 = await run2.drain_events()

    assert len(ev1) > 0
    assert len(ev2) > 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_after_drain_stream_is_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="post-drain", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    remaining = []
    async for ev in run.stream_events():
        remaining.append(ev)
    assert len(remaining) == 0
    rt.free()


@pytest.mark.asyncio
async def test_streaming_stream_events_is_async_generator():
    from inspect import isasyncgenfunction
    from ancora.run import Run
    assert isasyncgenfunction(Run.stream_events)


@pytest.mark.asyncio
async def test_streaming_repr_contains_run_id():
    rt = ancora.Runtime()
    spec = AgentSpec(name="repr-stream", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id in repr(run)
    await run.drain_events()
    rt.free()
