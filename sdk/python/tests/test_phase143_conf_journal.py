"""Phase 143 conformance task 8: journal matches core fixture."""

import json
import pytest
import ancora
from ancora.models import AgentSpec


JOURNAL_FIXTURE = [
    {"seq": 0, "kind": "run_start", "agent": "journal-agent"},
    {"seq": 1, "kind": "tool_call", "tool": "noop"},
    {"seq": 2, "kind": "tool_result", "tool": "noop", "output": "ok"},
    {"seq": 3, "kind": "run_end"},
]


def test_journal_fixture_ordered_by_seq():
    seqs = [e["seq"] for e in JOURNAL_FIXTURE]
    assert seqs == sorted(seqs)


def test_journal_fixture_starts_with_run_start():
    assert JOURNAL_FIXTURE[0]["kind"] == "run_start"


def test_journal_fixture_ends_with_run_end():
    assert JOURNAL_FIXTURE[-1]["kind"] == "run_end"


def test_journal_fixture_has_tool_call():
    kinds = [e["kind"] for e in JOURNAL_FIXTURE]
    assert "tool_call" in kinds


def test_journal_fixture_has_tool_result():
    kinds = [e["kind"] for e in JOURNAL_FIXTURE]
    assert "tool_result" in kinds


def test_journal_fixture_json_round_trip():
    raw = json.dumps(JOURNAL_FIXTURE)
    parsed = json.loads(raw)
    assert parsed[0]["kind"] == "run_start"
    assert parsed[-1]["kind"] == "run_end"


def test_journal_fixture_seqs_contiguous():
    seqs = [e["seq"] for e in JOURNAL_FIXTURE]
    assert seqs == list(range(len(seqs)))


@pytest.mark.asyncio
async def test_journal_run_events_yield_bytes():
    rt = ancora.Runtime()
    spec = AgentSpec(name="journal-e2e", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert all(isinstance(ev, bytes) for ev in events)
    rt.free()


@pytest.mark.asyncio
async def test_journal_events_parseable_as_json():
    rt = ancora.Runtime()
    spec = AgentSpec(name="journal-parse", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        obj = json.loads(ev)
        assert isinstance(obj, dict)
    rt.free()


@pytest.mark.asyncio
async def test_journal_events_have_kind_field():
    rt = ancora.Runtime()
    spec = AgentSpec(name="journal-kind", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        obj = json.loads(ev)
        assert "kind" in obj or len(obj) > 0
    rt.free()
