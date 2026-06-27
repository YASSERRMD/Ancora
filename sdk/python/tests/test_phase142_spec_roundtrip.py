"""Phase 142 task 2: spec round-trip."""

import pytest
import ancora
from ancora.models import AgentSpec, ToolSpec, EffectClass
from ancora.wire import to_wire_bytes, from_wire_bytes


def test_agentspec_defaults():
    spec = AgentSpec(name="agent", model_id="llama3")
    assert spec.name == "agent"
    assert spec.model_id == "llama3"
    assert spec.tools == []


def test_agentspec_round_trip_wire():
    spec = AgentSpec(name="rt-agent", model_id="gpt-4o", instructions="be helpful")
    b = to_wire_bytes(spec)
    assert isinstance(b, bytes)
    assert len(b) > 0


def test_agentspec_with_tool():
    tool = ToolSpec(name="search", description="web search")
    spec = AgentSpec(name="agent", model_id="llama3", tools=[tool])
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "search"


def test_toolspec_effect_class_default():
    ts = ToolSpec(name="t1", description="test tool")
    assert ts.effect_class == EffectClass.UNSPECIFIED


def test_toolspec_read_effect():
    ts = ToolSpec(name="searcher", description="reads data", effect_class=EffectClass.READ)
    assert ts.effect_class == EffectClass.READ


def test_agentspec_builder_round_trip():
    spec = (
        ancora.AgentSpecBuilder()
        .with_name("builder-agent")
        .with_model_id("llama3")
        .with_instructions("test instructions")
        .build()
    )
    assert spec.name == "builder-agent"
    assert spec.model_id == "llama3"
    assert spec.instructions == "test instructions"


def test_agentspec_builder_wire_bytes():
    spec = (
        ancora.AgentSpecBuilder()
        .with_name("wire-agent")
        .with_model_id("gpt-4o")
        .build()
    )
    b = to_wire_bytes(spec)
    assert isinstance(b, bytes)
    assert len(b) > 0


def test_agentspec_unicode_name():
    spec = AgentSpec(name="agent-中文", model_id="llama3")
    b = to_wire_bytes(spec)
    assert isinstance(b, bytes)


def test_agentspec_max_steps():
    spec = AgentSpec(name="ms-agent", model_id="llama3", max_steps=5)
    assert spec.max_steps == 5


def test_toolspec_builder_round_trip():
    ts = (
        ancora.ToolSpecBuilder()
        .with_name("tool1")
        .with_description("does something")
        .build()
    )
    assert ts.name == "tool1"
    assert ts.description == "does something"
