"""Tests for event JSON format produced by InnerRun."""

import json
import pytest
import ancora


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


def poll_all(rt, run_id):
    events = []
    while True:
        ev = rt.poll_run(run_id)
        if ev is None:
            break
        events.append(bytes(ev).decode("utf-8"))
    return events


def test_started_event_is_valid_json(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    ev = rt.poll_run(run_id)
    assert ev is not None
    parsed = json.loads(bytes(ev).decode("utf-8"))
    assert parsed["kind"] == "started"
    assert parsed["run_id"] == run_id


def test_completed_event_is_valid_json(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    evs = poll_all(rt, run_id)
    last = json.loads(evs[-1])
    assert last["kind"] == "completed"
    assert last["run_id"] == run_id


def test_resumed_event_is_valid_json(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    poll_all(rt, run_id)
    rt.resume_run(run_id, b"approved")
    ev = rt.poll_run(run_id)
    assert ev is not None
    parsed = json.loads(bytes(ev).decode("utf-8"))
    assert parsed["kind"] == "resumed"
    assert parsed["run_id"] == run_id
    assert "approved" in parsed["decision"]


def test_event_ordering(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    evs = poll_all(rt, run_id)
    assert len(evs) == 5
    assert json.loads(evs[0])["kind"] == "started"
    assert json.loads(evs[-1])["kind"] == "completed"
    token_kinds = [json.loads(e)["kind"] for e in evs[1:-1]]
    assert token_kinds == ["token", "token", "token"]


def test_resume_event_ordering(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    poll_all(rt, run_id)
    rt.resume_run(run_id, b"ok")
    evs = poll_all(rt, run_id)
    assert len(evs) == 2
    assert json.loads(evs[0])["kind"] == "resumed"
    assert json.loads(evs[1])["kind"] == "completed"
