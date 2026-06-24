"""Tests for wire serialization in ancora.wire."""

import json
import ancora
from ancora.models import AgentSpec, EffectClass, RetryPolicy, ToolSpec
from ancora.wire import from_wire_bytes, to_wire_bytes


def test_to_wire_bytes_returns_bytes():
    spec = AgentSpec(name="agent", model_id="llama3", instructions="go")
    data = to_wire_bytes(spec)
    assert isinstance(data, bytes)


def test_to_wire_bytes_is_valid_json():
    spec = AgentSpec(name="agent", model_id="llama3")
    data = to_wire_bytes(spec)
    parsed = json.loads(data)
    assert parsed["name"] == "agent"
    assert parsed["model_id"] == "llama3"


def test_from_wire_bytes_round_trip():
    spec = AgentSpec(name="agent", model_id="llama3", instructions="do stuff")
    data = to_wire_bytes(spec)
    recovered = from_wire_bytes(data)
    assert recovered.name == spec.name
    assert recovered.model_id == spec.model_id
    assert recovered.instructions == spec.instructions


def test_round_trip_with_tools():
    tool = ToolSpec(name="search", description="web search", effect_class=EffectClass.READ)
    spec = AgentSpec(name="agent", model_id="m", tools=[tool])
    data = to_wire_bytes(spec)
    recovered = from_wire_bytes(data)
    assert len(recovered.tools) == 1
    assert recovered.tools[0].name == "search"
    assert recovered.tools[0].effect_class == EffectClass.READ


def test_round_trip_with_retry_policy():
    retry = RetryPolicy(max_attempts=3, initial_backoff_ms=200, jitter=0.1)
    spec = AgentSpec(name="agent", model_id="m", model_retry=retry)
    data = to_wire_bytes(spec)
    recovered = from_wire_bytes(data)
    assert recovered.model_retry is not None
    assert recovered.model_retry.max_attempts == 3
    assert recovered.model_retry.jitter == 0.1


def test_to_wire_bytes_uses_snake_case_keys():
    spec = AgentSpec(name="agent", model_id="llama3", max_steps=10)
    parsed = json.loads(to_wire_bytes(spec))
    assert "model_id" in parsed
    assert "max_steps" in parsed


def test_wire_bytes_accessible_from_ancora_namespace():
    spec = ancora.AgentSpec(name="a", model_id="m")
    data = ancora.to_wire_bytes(spec)
    recovered = ancora.from_wire_bytes(data)
    assert recovered.name == "a"
