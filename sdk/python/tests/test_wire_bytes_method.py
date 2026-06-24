"""Tests for AgentSpec.wire_bytes() convenience method."""

import json
from ancora.models import AgentSpec, ToolSpec, EffectClass


def test_wire_bytes_returns_bytes():
    spec = AgentSpec(name="a", model_id="m")
    assert isinstance(spec.wire_bytes(), bytes)


def test_wire_bytes_is_valid_json():
    spec = AgentSpec(name="a", model_id="m")
    parsed = json.loads(spec.wire_bytes())
    assert parsed["name"] == "a"
    assert parsed["model_id"] == "m"


def test_wire_bytes_with_tools():
    spec = AgentSpec(name="a", model_id="m", tools=[ToolSpec(name="t")])
    parsed = json.loads(spec.wire_bytes())
    assert len(parsed["tools"]) == 1
    assert parsed["tools"][0]["name"] == "t"


def test_wire_bytes_matches_to_wire_bytes():
    from ancora.wire import to_wire_bytes
    spec = AgentSpec(name="x", model_id="y", instructions="z")
    assert spec.wire_bytes() == to_wire_bytes(spec)
