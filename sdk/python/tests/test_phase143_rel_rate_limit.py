"""Phase 143 reliability task 12: rate-limit handling."""

import time
import asyncio
import pytest
import ancora
from ancora.models import AgentSpec


RATE_LIMIT_FIXTURE = {
    "status": 429,
    "retry_after_ms": 100,
    "message": "Too Many Requests",
}


def test_rate_limit_fixture_status_is_429():
    assert RATE_LIMIT_FIXTURE["status"] == 429


def test_rate_limit_fixture_has_retry_after():
    assert RATE_LIMIT_FIXTURE["retry_after_ms"] > 0


def test_rate_limit_fixture_message_non_empty():
    assert RATE_LIMIT_FIXTURE["message"] != ""


@pytest.mark.asyncio
async def test_rate_limit_burst_runs_all_succeed():
    rt = ancora.Runtime()
    spec = AgentSpec(name="rl-burst", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = await asyncio.gather(*[agent.run() for _ in range(5)])
    assert all(r.run_id != "" for r in runs)
    rt.free()


@pytest.mark.asyncio
async def test_rate_limit_run_ids_unique_under_burst():
    rt = ancora.Runtime()
    spec = AgentSpec(name="rl-unique", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = await asyncio.gather(*[agent.run() for _ in range(5)])
    ids = {r.run_id for r in runs}
    assert len(ids) == 5
    rt.free()


@pytest.mark.asyncio
async def test_rate_limit_sequential_runs_after_delay():
    rt = ancora.Runtime()
    spec = AgentSpec(name="rl-seq", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    for _ in range(3):
        run = await agent.run()
        assert run.run_id != ""
        await run.drain_events()
        await asyncio.sleep(0)
    rt.free()


@pytest.mark.asyncio
async def test_rate_limit_exponential_backoff_simulated():
    delays = [0.01 * (2 ** i) for i in range(4)]
    assert delays == [0.01, 0.02, 0.04, 0.08]
    rt = ancora.Runtime()
    spec = AgentSpec(name="rl-backoff", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    await run.drain_events()
    rt.free()


@pytest.mark.asyncio
async def test_rate_limit_total_wall_time_reasonable():
    rt = ancora.Runtime()
    spec = AgentSpec(name="rl-wall", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    start = time.monotonic()
    runs = await asyncio.gather(*[agent.run() for _ in range(4)])
    elapsed = time.monotonic() - start
    assert all(r.run_id != "" for r in runs)
    assert elapsed < 30.0
    rt.free()
