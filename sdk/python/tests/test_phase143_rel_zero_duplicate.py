"""Phase 143 reliability task 10: zero duplicate side effects."""

import pytest
import ancora
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec


side_effects: list = []


@tool
def effect_once(label: str) -> str:
    """Record side effect once."""
    side_effects.append(label)
    return f"recorded:{label}"


def setup_function():
    side_effects.clear()


def test_zero_dup_tool_called_once_records_once():
    side_effects.clear()
    effect_once.call_with_json('{"label": "A"}')
    assert side_effects.count("A") == 1


def test_zero_dup_two_calls_produce_two_records():
    side_effects.clear()
    effect_once.call_with_json('{"label": "B"}')
    effect_once.call_with_json('{"label": "C"}')
    assert len(side_effects) == 2
    assert "B" in side_effects
    assert "C" in side_effects


def test_zero_dup_distinct_labels_no_dedup():
    side_effects.clear()
    for ch in "DEFGH":
        effect_once.call_with_json(f'{{"label": "{ch}"}}')
    assert len(side_effects) == 5


def test_zero_dup_registry_dispatch_once():
    side_effects.clear()
    reg = ToolRegistry()
    reg.register(effect_once)
    reg.dispatch("effect_once", '{"label": "X"}')
    assert side_effects.count("X") == 1


@pytest.mark.asyncio
async def test_zero_dup_runs_have_unique_ids():
    rt = ancora.Runtime()
    spec = AgentSpec(name="dup-check", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = [await agent.run() for _ in range(5)]
    ids = [r.run_id for r in runs]
    assert len(set(ids)) == len(ids)
    rt.free()


@pytest.mark.asyncio
async def test_zero_dup_drain_twice_second_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="drain-once", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    ev1 = await run.drain_events()
    ev2 = await run.drain_events()
    assert len(ev1) > 0
    assert len(ev2) == 0
    rt.free()


@pytest.mark.asyncio
async def test_zero_dup_no_duplicate_run_ids_across_runtimes():
    ids = set()
    for _ in range(4):
        rt = ancora.Runtime()
        spec = AgentSpec(name="rt-uniq", model_id="llama3")
        run = await ancora.Agent(rt, spec).run()
        assert run.run_id not in ids
        ids.add(run.run_id)
        await run.drain_events()
        rt.free()


def test_zero_dup_effect_list_cleared_between_tests():
    assert len(side_effects) == 0
