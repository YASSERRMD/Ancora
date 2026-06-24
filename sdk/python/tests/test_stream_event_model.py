"""Tests for the StreamEvent Pydantic model."""

import pytest

from ancora.models import StreamEvent


def test_stream_event_defaults():
    ev = StreamEvent()
    assert ev.kind == ""
    assert ev.run_id == ""
    assert ev.text is None
    assert ev.spec is None
    assert ev.decision is None


def test_stream_event_from_started_bytes():
    raw = b'{"kind":"started","run_id":"abc","spec":"{}"}'
    ev = StreamEvent.from_bytes(raw)
    assert ev.kind == "started"
    assert ev.run_id == "abc"
    assert ev.spec == "{}"


def test_stream_event_from_token_bytes():
    raw = b'{"kind":"token","run_id":"r1","text":"hi"}'
    ev = StreamEvent.from_bytes(raw)
    assert ev.kind == "token"
    assert ev.text == "hi"


def test_stream_event_from_completed_bytes():
    raw = b'{"kind":"completed","run_id":"r2"}'
    ev = StreamEvent.from_bytes(raw)
    assert ev.kind == "completed"
    assert ev.text is None


def test_stream_event_extra_fields_allowed():
    raw = b'{"kind":"custom","run_id":"r","extra":"data"}'
    ev = StreamEvent.from_bytes(raw)
    assert ev.kind == "custom"


def test_stream_event_repr_contains_kind():
    ev = StreamEvent(kind="token", run_id="r", text="x")
    r = repr(ev)
    assert "token" in r
