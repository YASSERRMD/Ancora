"""Cross-language conformance: verifier scenario -- Python (offline)."""
import json

XLANG_VERIFIER_RUN_ID = "xlv-python"

XLANG_VERIFIER_EVENTS = [
    {"kind": "started",   "run_id": XLANG_VERIFIER_RUN_ID},
    {"kind": "activity",  "run_id": XLANG_VERIFIER_RUN_ID, "activity_key": "drafter"},
    {"kind": "activity",  "run_id": XLANG_VERIFIER_RUN_ID, "activity_key": "verifier"},
    {"kind": "completed", "run_id": XLANG_VERIFIER_RUN_ID, "output": {"verdict": "approved"}},
]


def test_xlang_python_verifier_started_first():
    assert XLANG_VERIFIER_EVENTS[0]["kind"] == "started"


def test_xlang_python_verifier_completed_last():
    assert XLANG_VERIFIER_EVENTS[-1]["kind"] == "completed"


def test_xlang_python_verifier_drafter_before_verifier():
    keys = [e["activity_key"] for e in XLANG_VERIFIER_EVENTS if e["kind"] == "activity"]
    assert keys == ["drafter", "verifier"]


def test_xlang_python_verifier_output_has_verdict():
    last = XLANG_VERIFIER_EVENTS[-1]
    assert last["output"]["verdict"] == "approved"


def test_xlang_python_verifier_run_id_consistent():
    for ev in XLANG_VERIFIER_EVENTS:
        assert ev["run_id"] == XLANG_VERIFIER_RUN_ID
