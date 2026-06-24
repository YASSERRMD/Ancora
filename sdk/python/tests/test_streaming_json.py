"""Tests that streaming events contain valid JSON."""

import json

import ancora


async def test_all_stream_events_are_valid_json():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    async for raw in run.stream_events():
        parsed = json.loads(raw)
        assert "kind" in parsed
        assert "run_id" in parsed

    rt.free()


async def test_token_events_have_text_field():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    async for raw in run.stream_events():
        parsed = json.loads(raw)
        if parsed["kind"] == "token":
            assert "text" in parsed
            assert isinstance(parsed["text"], str)

    rt.free()


async def test_stream_event_from_bytes_parses_token():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    found_token = False
    async for raw in run.stream_events():
        ev = ancora.StreamEvent.from_bytes(raw)
        if ev.kind == "token":
            assert ev.text is not None
            found_token = True

    assert found_token
    rt.free()


async def test_stream_event_from_bytes_parses_started():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    events = []
    async for raw in run.stream_events():
        events.append(ancora.StreamEvent.from_bytes(raw))

    started = [e for e in events if e.kind == "started"]
    assert len(started) == 1
    assert started[0].run_id == run.run_id

    rt.free()
