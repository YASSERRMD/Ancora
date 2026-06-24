"""Tests that human-in-the-loop journal matches CORE_FIXTURE."""

import json

import ancora
from ancora.conformance import CORE_FIXTURE
from ancora.models import StreamEvent


async def test_journal_hitl_resume_event_kinds():
    fixture = CORE_FIXTURE["human_in_loop"]
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b"approved")
    events = await run.drain_events()
    actual_kinds = [json.loads(e)["kind"] for e in events]
    assert actual_kinds == fixture["resume_event_kinds"]
    rt.free()


async def test_journal_hitl_resumed_has_decision():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b"yes")
    events = await run.drain_events()
    resumed = StreamEvent.from_bytes(events[0])
    assert resumed.kind == "resumed"
    assert resumed.decision is not None
    assert "yes" in resumed.decision
    rt.free()


async def test_journal_hitl_completed_after_resume():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    await run.drain_events()
    await run.resume(b"ok")
    events = await run.drain_events()
    last = StreamEvent.from_bytes(events[-1])
    assert last.is_completed
    rt.free()
