"""Tests that the journal event sequence matches the CORE_FIXTURE."""

import json

import ancora
from ancora.conformance import CORE_FIXTURE
from ancora.models import StreamEvent


async def test_journal_single_run_event_kinds():
    fixture = CORE_FIXTURE["single_run"]
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    actual_kinds = [json.loads(e)["kind"] for e in events]
    assert actual_kinds == fixture["event_kinds"]
    rt.free()


async def test_journal_single_run_event_count():
    fixture = CORE_FIXTURE["single_run"]
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) == fixture["event_count"]
    rt.free()


async def test_journal_first_event_is_started():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    first = StreamEvent.from_bytes(events[0])
    assert first.is_started
    assert first.run_id == run.run_id
    rt.free()


async def test_journal_last_event_is_completed():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    last = StreamEvent.from_bytes(events[-1])
    assert last.is_completed
    assert last.run_id == run.run_id
    rt.free()


async def test_journal_middle_events_are_tokens():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    middle = [StreamEvent.from_bytes(e) for e in events[1:-1]]
    assert all(e.is_token for e in middle)
    assert all(e.text is not None for e in middle)
    rt.free()


async def test_journal_started_contains_spec():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="fixture-agent", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    started = StreamEvent.from_bytes(events[0])
    assert started.spec is not None
    assert "fixture-agent" in started.spec
    rt.free()


async def test_journal_run_ids_consistent():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    run_ids = [json.loads(e)["run_id"] for e in events]
    assert all(r == run.run_id for r in run_ids)
    rt.free()
