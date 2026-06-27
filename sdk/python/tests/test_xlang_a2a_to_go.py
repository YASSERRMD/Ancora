"""Cross-language A2A: Python hands off to Go over A2A (offline fixture)."""
import json

HANDOFF_RUN_ID = "a2a-py-go-001"

A2A_ENVELOPE = {
    "protocol": "a2a/1.0",
    "sender": {"lang": "python", "sdk_version": "0.3.0"},
    "recipient": {"lang": "go", "sdk_version": "0.3.0"},
    "run_id": HANDOFF_RUN_ID,
    "payload": {
        "task": "verify",
        "draft": "Python produced this draft",
        "handoff_reason": "verifier runs in Go",
    },
}

A2A_RESPONSE = {
    "protocol": "a2a/1.0",
    "sender": {"lang": "go"},
    "recipient": {"lang": "python"},
    "run_id": HANDOFF_RUN_ID,
    "payload": {"verdict": "approved", "score": 0.97},
}


def test_a2a_py_go_envelope_has_protocol():
    assert A2A_ENVELOPE["protocol"] == "a2a/1.0"


def test_a2a_py_go_envelope_sender_is_python():
    assert A2A_ENVELOPE["sender"]["lang"] == "python"


def test_a2a_py_go_envelope_recipient_is_go():
    assert A2A_ENVELOPE["recipient"]["lang"] == "go"


def test_a2a_py_go_run_id_matches_in_response():
    assert A2A_RESPONSE["run_id"] == A2A_ENVELOPE["run_id"]


def test_a2a_py_go_response_sender_is_go():
    assert A2A_RESPONSE["sender"]["lang"] == "go"


def test_a2a_py_go_response_has_verdict():
    assert A2A_RESPONSE["payload"]["verdict"] == "approved"


def test_a2a_py_go_envelope_serialises_to_json():
    raw = json.dumps(A2A_ENVELOPE)
    decoded = json.loads(raw)
    assert decoded["protocol"] == "a2a/1.0"


def test_a2a_py_go_payload_has_handoff_reason():
    assert "handoff_reason" in A2A_ENVELOPE["payload"]
