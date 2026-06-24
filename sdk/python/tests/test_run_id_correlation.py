"""Tests that run ID from Run handle matches run_id in events."""

import json
import pytest
import ancora
from ancora.agent import Agent
from ancora.models import AgentSpec


@pytest.fixture
def rt():
    runtime = ancora.Runtime()
    yield runtime
    runtime.free()


async def test_run_id_matches_started_event(rt):
    spec = AgentSpec(name="a", model_id="m")
    run = await Agent(rt, spec).run()
    events = await run.drain_events()
    started = next(e for e in events if b"started" in e)
    parsed = json.loads(started.decode("utf-8"))
    assert parsed["run_id"] == run.run_id


async def test_run_id_matches_completed_event(rt):
    spec = AgentSpec(name="a", model_id="m")
    run = await Agent(rt, spec).run()
    events = await run.drain_events()
    completed = next(e for e in events if b"completed" in e)
    parsed = json.loads(completed.decode("utf-8"))
    assert parsed["run_id"] == run.run_id


async def test_run_id_matches_resumed_event(rt):
    spec = AgentSpec(name="a", model_id="m")
    run = await Agent(rt, spec).run()
    await run.drain_events()
    await run.resume(b"ok")
    events = await run.drain_events()
    resumed = next(e for e in events if b"resumed" in e)
    parsed = json.loads(resumed.decode("utf-8"))
    assert parsed["run_id"] == run.run_id
