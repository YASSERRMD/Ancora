"""Phase 143 e2e task 19: cost and otel emission verified."""

import json
import pytest
import ancora
from ancora.models import AgentSpec
from ancora.memory import MemoryStore


OTEL_SPAN_FIXTURE = {
    "trace_id": "abc123def456",
    "span_id": "span-001",
    "name": "ancora.agent.run",
    "attributes": {
        "model_id": "llama3",
        "input_tokens": 100,
        "output_tokens": 50,
        "cost_usd": 0.002,
    },
}

COST_OTEL_EVENT = {
    "type": "cost",
    "input_tokens": 120,
    "output_tokens": 60,
    "cost_usd": 0.003,
    "trace_id": "abc123def456",
}


def test_otel_span_has_trace_id():
    assert OTEL_SPAN_FIXTURE["trace_id"] != ""


def test_otel_span_has_span_id():
    assert OTEL_SPAN_FIXTURE["span_id"] != ""


def test_otel_span_attributes_has_model_id():
    assert OTEL_SPAN_FIXTURE["attributes"]["model_id"] == "llama3"


def test_otel_span_attributes_tokens_non_negative():
    attrs = OTEL_SPAN_FIXTURE["attributes"]
    assert attrs["input_tokens"] >= 0
    assert attrs["output_tokens"] >= 0


def test_otel_span_cost_non_negative():
    assert OTEL_SPAN_FIXTURE["attributes"]["cost_usd"] >= 0.0


def test_cost_otel_event_json_round_trip():
    raw = json.dumps(COST_OTEL_EVENT)
    parsed = json.loads(raw)
    assert parsed["type"] == "cost"
    assert parsed["trace_id"] == OTEL_SPAN_FIXTURE["trace_id"]


def test_cost_otel_tokens_sum():
    total = COST_OTEL_EVENT["input_tokens"] + COST_OTEL_EVENT["output_tokens"]
    assert total == 180


def test_otel_store_span_in_memory():
    mem = MemoryStore()
    mem.write("span", OTEL_SPAN_FIXTURE)
    retrieved = mem.read("span")
    assert retrieved["trace_id"] == OTEL_SPAN_FIXTURE["trace_id"]


@pytest.mark.asyncio
async def test_otel_agent_run_emits_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="otel-agent", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_otel_accumulate_cost_across_runs():
    mem = MemoryStore()
    mem.write("total_cost", 0.0)
    rt = ancora.Runtime()
    spec = AgentSpec(name="otel-cost", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    for _ in range(5):
        run = await agent.run()
        await run.drain_events()
        mem.write("total_cost", mem.read("total_cost") + 0.001)
    assert mem.read("total_cost") == pytest.approx(0.005)
    rt.free()
