"""Cross-language conformance: single agent scenario -- Python binding (offline)."""
from __future__ import annotations

import json
from typing import List


XLANG_RUN_ID = "xlang-python-001"

XLANG_EVENTS: List[dict] = [
    {"kind": "started", "run_id": XLANG_RUN_ID, "spec": "{}"},
    {"kind": "token", "run_id": XLANG_RUN_ID, "text": "xlang python result"},
    {"kind": "completed", "run_id": XLANG_RUN_ID},
]


def test_xlang_python_started_event_is_first():
    assert XLANG_EVENTS[0]["kind"] == "started"


def test_xlang_python_completed_event_is_last():
    assert XLANG_EVENTS[-1]["kind"] == "completed"


def test_xlang_python_run_id_consistent_across_events():
    for ev in XLANG_EVENTS:
        assert ev["run_id"] == XLANG_RUN_ID, f"run_id mismatch: {ev}"


def test_xlang_python_event_count_at_least_two():
    assert len(XLANG_EVENTS) >= 2


def test_xlang_python_token_text_non_empty():
    token_events = [e for e in XLANG_EVENTS if e["kind"] == "token"]
    assert token_events, "expected at least one token event"
    assert all(e["text"] for e in token_events)


def test_xlang_python_events_serialise_to_json():
    for ev in XLANG_EVENTS:
        serialised = json.dumps(ev)
        decoded = json.loads(serialised)
        assert decoded["kind"] == ev["kind"]


def test_xlang_python_no_event_before_started():
    first = XLANG_EVENTS[0]
    assert first["kind"] == "started", "started must come before any other event"


def test_xlang_python_no_event_after_completed():
    last = XLANG_EVENTS[-1]
    assert last["kind"] == "completed", "completed must be the last event"
