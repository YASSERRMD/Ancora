"""Tests for Pydantic models in ancora.models."""

import pytest
import ancora
from ancora.models import AgentSpec, EffectClass, RetryPolicy, ToolSpec


def test_effect_class_values():
    assert EffectClass.UNSPECIFIED == 0
    assert EffectClass.PURE == 1
    assert EffectClass.READ == 2
    assert EffectClass.WRITE == 3


def test_effect_class_is_int_enum():
    assert isinstance(EffectClass.PURE, int)


def test_tool_spec_defaults():
    t = ToolSpec()
    assert t.name == ""
    assert t.description == ""
    assert t.effect_class == EffectClass.UNSPECIFIED


def test_tool_spec_with_values():
    t = ToolSpec(name="search", description="search the web", effect_class=EffectClass.READ)
    assert t.name == "search"
    assert t.description == "search the web"
    assert t.effect_class == EffectClass.READ


def test_retry_policy_defaults():
    r = RetryPolicy()
    assert r.max_attempts == 0
    assert r.initial_backoff_ms == 0
    assert r.max_backoff_ms == 0
    assert r.jitter == 0.0


def test_retry_policy_jitter_bounds():
    with pytest.raises(Exception):
        RetryPolicy(jitter=1.5)
    with pytest.raises(Exception):
        RetryPolicy(jitter=-0.1)


def test_agent_spec_defaults():
    spec = AgentSpec()
    assert spec.name == ""
    assert spec.model_id == ""
    assert spec.tools == []
    assert spec.model_retry is None


def test_agent_spec_with_name_and_model():
    spec = AgentSpec(name="my-agent", model_id="llama3", instructions="do the thing")
    assert spec.name == "my-agent"
    assert spec.model_id == "llama3"
    assert spec.instructions == "do the thing"


def test_agent_spec_with_tools():
    tool = ToolSpec(name="calc", description="calculator")
    spec = AgentSpec(name="agent", model_id="m", tools=[tool])
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "calc"


def test_agent_spec_max_steps_non_negative():
    with pytest.raises(Exception):
        AgentSpec(max_steps=-1)


def test_agent_spec_with_retry_policy():
    retry = RetryPolicy(max_attempts=3, initial_backoff_ms=100)
    spec = AgentSpec(name="agent", model_id="m", model_retry=retry)
    assert spec.model_retry is not None
    assert spec.model_retry.max_attempts == 3


def test_tool_spec_rejects_extra_fields():
    with pytest.raises(Exception):
        ToolSpec(name="t", unknown_field="x")


def test_agent_spec_rejects_extra_fields():
    with pytest.raises(Exception):
        AgentSpec(name="a", bogus_key="v")
