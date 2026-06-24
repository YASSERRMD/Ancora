"""Tests for StreamEvent convenience properties."""

import ancora
from ancora.models import StreamEvent


def test_is_token_true_for_token_event():
    ev = StreamEvent(kind="token", run_id="r", text="hi")
    assert ev.is_token is True


def test_is_token_false_for_other_kinds():
    assert StreamEvent(kind="started").is_token is False
    assert StreamEvent(kind="completed").is_token is False
    assert StreamEvent(kind="resumed").is_token is False


def test_is_started_true_for_started():
    ev = StreamEvent(kind="started", run_id="r", spec="{}")
    assert ev.is_started is True


def test_is_started_false_for_other_kinds():
    assert StreamEvent(kind="token", text="t").is_started is False
    assert StreamEvent(kind="completed").is_started is False


def test_is_completed_true_for_completed():
    ev = StreamEvent(kind="completed", run_id="r")
    assert ev.is_completed is True


def test_is_completed_false_for_other_kinds():
    assert StreamEvent(kind="started").is_completed is False
    assert StreamEvent(kind="token", text="x").is_completed is False


async def test_stream_events_with_helper_properties():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    events = []
    async for raw in run.stream_events():
        events.append(StreamEvent.from_bytes(raw))

    assert events[0].is_started
    assert events[-1].is_completed
    token_events = [e for e in events if e.is_token]
    assert len(token_events) == 3

    rt.free()
