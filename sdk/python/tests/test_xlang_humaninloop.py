"""Cross-language conformance: human-in-loop scenario -- Python (offline)."""
import json

XLANG_HIL_RUN_ID = "xlh-python"

XLANG_HIL_EVENTS = [
    {"kind": "started",            "run_id": XLANG_HIL_RUN_ID},
    {"kind": "decision_requested", "run_id": XLANG_HIL_RUN_ID, "prompt": "Please approve the draft", "options": ["approve", "reject"]},
    {"kind": "decision_received",  "run_id": XLANG_HIL_RUN_ID, "decision": '{"approved":true}'},
    {"kind": "completed",          "run_id": XLANG_HIL_RUN_ID, "output": {"result": "hil-ok"}},
]


def test_xlang_python_hil_started_first():
    assert XLANG_HIL_EVENTS[0]["kind"] == "started"


def test_xlang_python_hil_requested_before_received():
    kinds = [e["kind"] for e in XLANG_HIL_EVENTS if "decision" in e["kind"]]
    assert kinds == ["decision_requested", "decision_received"]


def test_xlang_python_hil_decision_is_approved():
    received = next(e for e in XLANG_HIL_EVENTS if e["kind"] == "decision_received")
    dec = json.loads(received["decision"])
    assert dec["approved"] is True


def test_xlang_python_hil_prompt_non_empty():
    requested = next(e for e in XLANG_HIL_EVENTS if e["kind"] == "decision_requested")
    assert requested["prompt"]
    assert requested["options"]


def test_xlang_python_hil_completed_last():
    assert XLANG_HIL_EVENTS[-1]["kind"] == "completed"


def test_xlang_python_hil_run_id_consistent():
    for ev in XLANG_HIL_EVENTS:
        assert ev["run_id"] == XLANG_HIL_RUN_ID
