"""Tests for Run.stream_tokens and Run.stream_events."""

import json

import ancora


async def test_stream_tokens_yields_strings():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    tokens = []
    async for t in run.stream_tokens():
        tokens.append(t)

    assert tokens == ["Hello", " ", "world"]
    rt.free()


async def test_stream_tokens_concatenated():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    result = "".join([t async for t in run.stream_tokens()])
    assert result == "Hello world"
    rt.free()


async def test_stream_events_includes_started_and_completed():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    events = [json.loads(ev) async for ev in run.stream_events()]
    kinds = [e["kind"] for e in events]

    assert kinds[0] == "started"
    assert kinds[-1] == "completed"
    rt.free()


async def test_stream_events_includes_token_events():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    events = [json.loads(ev) async for ev in run.stream_events()]
    token_events = [e for e in events if e["kind"] == "token"]

    assert len(token_events) == 3
    assert token_events[0]["text"] == "Hello"
    rt.free()


async def test_drain_events_after_stream_tokens_is_empty():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    async for _ in run.stream_tokens():
        pass

    remaining = await run.drain_events()
    assert remaining == []
    rt.free()
