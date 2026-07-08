"""Tests for AgentSpecBuilder and ToolSpecBuilder."""

import ancora
from ancora.builder import AgentSpecBuilder, ToolSpecBuilder
from ancora.models import EffectClass, RetryPolicy


def test_tool_spec_builder_name():
    t = ToolSpecBuilder().with_name("search").build()
    assert t.name == "search"


def test_tool_spec_builder_description():
    t = ToolSpecBuilder().with_description("web search").build()
    assert t.description == "web search"


def test_tool_spec_builder_effect_class():
    t = ToolSpecBuilder().with_effect_class(EffectClass.READ).build()
    assert t.effect_class == EffectClass.READ


def test_tool_spec_builder_chaining():
    t = (
        ToolSpecBuilder()
        .with_name("calc")
        .with_description("arithmetic")
        .with_effect_class(EffectClass.PURE)
        .build()
    )
    assert t.name == "calc"
    assert t.description == "arithmetic"
    assert t.effect_class == EffectClass.PURE


def test_agent_spec_builder_name_and_model():
    spec = AgentSpecBuilder().with_name("agent").with_model_id("llama3").build()
    assert spec.name == "agent"
    assert spec.model_id == "llama3"


def test_agent_spec_builder_instructions():
    spec = (
        AgentSpecBuilder()
        .with_name("agent")
        .with_model_id("llama3")
        .with_instructions("do the thing")
        .build()
    )
    assert spec.instructions == "do the thing"


def test_agent_spec_builder_with_tool():
    tool = ToolSpecBuilder().with_name("calc").build()
    spec = AgentSpecBuilder().with_name("agent").with_model_id("llama3").with_tool(tool).build()
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "calc"


def test_agent_spec_builder_with_multiple_tools():
    t1 = ToolSpecBuilder().with_name("tool1").build()
    t2 = ToolSpecBuilder().with_name("tool2").build()
    spec = (
        AgentSpecBuilder()
        .with_name("agent")
        .with_model_id("llama3")
        .with_tool(t1)
        .with_tool(t2)
        .build()
    )
    assert len(spec.tools) == 2


def test_agent_spec_builder_max_steps():
    spec = AgentSpecBuilder().with_name("agent").with_model_id("llama3").with_max_steps(10).build()
    assert spec.max_steps == 10


def test_agent_spec_builder_retry():
    retry = RetryPolicy(max_attempts=3)
    spec = (
        AgentSpecBuilder()
        .with_name("agent")
        .with_model_id("llama3")
        .with_model_retry(retry)
        .build()
    )
    assert spec.model_retry is not None
    assert spec.model_retry.max_attempts == 3


def test_builder_accessible_from_ancora_namespace():
    spec = ancora.AgentSpecBuilder().with_name("a").with_model_id("m").build()
    assert spec.name == "a"
