"""Tests for raw Runtime start_run/poll_run/resume_run operations."""

import pytest
import ancora


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


def test_start_run_returns_string(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    assert isinstance(run_id, str)
    assert len(run_id) > 0


def test_start_run_different_ids(rt):
    id1 = rt.start_run(b'{"name":"a","model_id":"m"}')
    id2 = rt.start_run(b'{"name":"a","model_id":"m"}')
    assert id1 != id2


def test_poll_run_returns_bytes_or_none(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    ev = rt.poll_run(run_id)
    assert ev is None or isinstance(ev, (bytes, list))


def test_poll_run_started_event(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    ev = rt.poll_run(run_id)
    assert ev is not None
    decoded = bytes(ev).decode("utf-8")
    assert '"kind":"started"' in decoded
    assert run_id in decoded


def test_poll_run_completed_event(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    ev = None
    for _ in range(10):
        ev = rt.poll_run(run_id)
        if ev is None:
            break
        last_ev = ev
    decoded = bytes(last_ev).decode("utf-8")
    assert '"kind":"completed"' in decoded


def test_poll_run_exhausted_returns_none(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    for _ in range(5):
        rt.poll_run(run_id)
    ev = rt.poll_run(run_id)
    assert ev is None


def test_resume_run_adds_resumed_event(rt):
    run_id = rt.start_run(b'{"name":"a","model_id":"m"}')
    for _ in range(5):
        rt.poll_run(run_id)
    rt.resume_run(run_id, b"yes")
    ev = rt.poll_run(run_id)
    assert ev is not None
    decoded = bytes(ev).decode("utf-8")
    assert '"kind":"resumed"' in decoded


def test_freed_runtime_start_run_raises(rt):
    rt.free()
    with pytest.raises(RuntimeError):
        rt.start_run(b'{}')


def test_freed_runtime_poll_run_raises(rt):
    run_id = ancora.Runtime().start_run(b'{"name":"a","model_id":"m"}')
    rt.free()
    with pytest.raises(RuntimeError):
        rt.poll_run(run_id)
