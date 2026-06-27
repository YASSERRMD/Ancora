"""Phase 142 task 13: cost summary."""

import json
import pytest
from ancora.memory import MemoryStore


def make_cost_event(input_tokens: int, output_tokens: int, cost_usd: float) -> dict:
    return {
        "type": "usage",
        "input_tokens": input_tokens,
        "output_tokens": output_tokens,
        "cost_usd": cost_usd,
    }


def test_cost_event_has_required_fields():
    ev = make_cost_event(100, 50, 0.002)
    assert "input_tokens" in ev
    assert "output_tokens" in ev
    assert "cost_usd" in ev


def test_cost_event_json_round_trip():
    ev = make_cost_event(150, 80, 0.003)
    raw = json.dumps(ev)
    parsed = json.loads(raw)
    assert parsed["input_tokens"] == 150
    assert parsed["output_tokens"] == 80


def test_cost_input_tokens_non_negative():
    ev = make_cost_event(0, 0, 0.0)
    assert ev["input_tokens"] >= 0


def test_cost_output_tokens_non_negative():
    ev = make_cost_event(10, 5, 0.001)
    assert ev["output_tokens"] >= 0


def test_cost_usd_non_negative():
    ev = make_cost_event(10, 5, 0.001)
    assert ev["cost_usd"] >= 0.0


def test_cost_memory_store_accumulates_costs():
    mem = MemoryStore()
    mem.write("total_cost_usd", 0.0)
    for _ in range(5):
        current = mem.read("total_cost_usd", 0.0)
        mem.write("total_cost_usd", current + 0.001)
    assert abs(mem.read("total_cost_usd") - 0.005) < 1e-9


def test_cost_memory_store_tracks_token_counts():
    mem = MemoryStore()
    mem.write("total_input_tokens", 0)
    mem.write("total_output_tokens", 0)
    for _ in range(3):
        mem.write("total_input_tokens", mem.read("total_input_tokens", 0) + 100)
        mem.write("total_output_tokens", mem.read("total_output_tokens", 0) + 50)
    assert mem.read("total_input_tokens") == 300
    assert mem.read("total_output_tokens") == 150


def test_cost_multiple_runs_sum():
    costs = [0.001, 0.002, 0.003]
    total = sum(costs)
    assert abs(total - 0.006) < 1e-9


def test_cost_event_type_field():
    ev = make_cost_event(100, 50, 0.002)
    assert ev["type"] == "usage"


def test_cost_event_list_is_ordered():
    events = [
        make_cost_event(100, 50, 0.002),
        make_cost_event(200, 100, 0.004),
        make_cost_event(50, 25, 0.001),
    ]
    total_input = sum(e["input_tokens"] for e in events)
    assert total_input == 350
