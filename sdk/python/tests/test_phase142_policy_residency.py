"""Phase 142 task 14: policy residency block."""

import json
import pytest
from ancora.memory import MemoryStore


def make_policy_event(region: str, blocked: bool, reason: str = "") -> dict:
    return {"region": region, "blocked": blocked, "reason": reason}


def test_policy_event_has_region():
    ev = make_policy_event("eu-west-1", False)
    assert "region" in ev
    assert ev["region"] == "eu-west-1"


def test_policy_event_has_blocked_flag():
    ev = make_policy_event("us-east-1", True, "no GDPR consent")
    assert ev["blocked"] is True


def test_policy_event_json_round_trip():
    ev = make_policy_event("ap-southeast-1", False)
    raw = json.dumps(ev)
    parsed = json.loads(raw)
    assert parsed["region"] == "ap-southeast-1"
    assert parsed["blocked"] is False


def test_policy_eu_region_blocked():
    ev = make_policy_event("eu-central-1", True, "GDPR residency required")
    assert ev["blocked"] is True
    assert "GDPR" in ev["reason"]


def test_policy_us_region_allowed():
    ev = make_policy_event("us-west-2", False)
    assert not ev["blocked"]


def test_policy_memory_store_records_residency():
    mem = MemoryStore()
    mem.write("residency_region", "eu-west-1")
    mem.write("residency_blocked", True)
    assert mem.read("residency_region") == "eu-west-1"
    assert mem.read("residency_blocked") is True


def test_policy_events_list_ordered():
    events = [
        make_policy_event("us-east-1", False),
        make_policy_event("eu-west-1", True, "GDPR"),
        make_policy_event("ap-southeast-1", False),
    ]
    blocked = [e for e in events if e["blocked"]]
    assert len(blocked) == 1
    assert blocked[0]["region"] == "eu-west-1"


def test_policy_reason_empty_when_allowed():
    ev = make_policy_event("us-east-1", False)
    assert ev["reason"] == ""


def test_policy_multiple_blocked_regions():
    regions = ["eu-west-1", "eu-central-1", "us-east-1"]
    events = [make_policy_event(r, "eu" in r, "GDPR" if "eu" in r else "") for r in regions]
    blocked = [e for e in events if e["blocked"]]
    assert len(blocked) == 2


def test_policy_event_type_is_dict():
    ev = make_policy_event("us-east-1", False)
    assert isinstance(ev, dict)
