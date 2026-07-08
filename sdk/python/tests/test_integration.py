"""Integration tests: build specs via builder and round-trip through wire."""

import json
import ancora
from ancora.builder import AgentSpecBuilder, ToolSpecBuilder
from ancora.models import EffectClass, RetryPolicy
from ancora.wire import from_wire_bytes, to_wire_bytes


def test_full_spec_round_trip():
    calc = ToolSpecBuilder().with_name("calc").with_effect_class(EffectClass.PURE).build()
    search = (
        ToolSpecBuilder()
        .with_name("search")
        .with_description("web search")
        .with_effect_class(EffectClass.READ)
        .build()
    )
    retry = RetryPolicy(max_attempts=3, initial_backoff_ms=100, jitter=0.2)
    spec = (
        AgentSpecBuilder()
        .with_name("research-agent")
        .with_model_id("llama3")
        .with_instructions("research and summarize")
        .with_tool(calc)
        .with_tool(search)
        .with_max_steps(20)
        .with_model_retry(retry)
        .build()
    )

    wire = to_wire_bytes(spec)
    recovered = from_wire_bytes(wire)

    assert recovered.name == "research-agent"
    assert recovered.model_id == "llama3"
    assert len(recovered.tools) == 2
    assert recovered.tools[0].name == "calc"
    assert recovered.tools[1].effect_class == EffectClass.READ
    assert recovered.max_steps == 20
    assert recovered.model_retry is not None
    assert recovered.model_retry.max_attempts == 3


def test_builder_instances_do_not_share_tools():
    b1 = AgentSpecBuilder().with_name("agent1").with_model_id("llama3")
    b2 = AgentSpecBuilder().with_name("agent2").with_model_id("llama3")
    b1.with_tool(ToolSpecBuilder().with_name("t1").build())
    s1 = b1.build()
    s2 = b2.build()
    assert len(s1.tools) == 1
    assert len(s2.tools) == 0


def test_wire_bytes_runtime_compatible_format():
    spec = AgentSpecBuilder().with_name("a").with_model_id("m").with_instructions("i").build()
    parsed = json.loads(to_wire_bytes(spec))
    assert "name" in parsed
    assert "model_id" in parsed
    assert "instructions" in parsed
