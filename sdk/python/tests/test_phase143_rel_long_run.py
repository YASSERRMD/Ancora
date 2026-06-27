"""Phase 143 reliability task 13: long-run stability."""

import asyncio
import pytest
import ancora
from ancora.models import AgentSpec


@pytest.mark.asyncio
async def test_long_run_fifty_sequential_runs():
    rt = ancora.Runtime()
    spec = AgentSpec(name="lr-seq", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    for i in range(50):
        run = await agent.run()
        events = await run.drain_events()
        assert run.run_id != "", f"Empty run ID at iteration {i}"
        assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_long_run_runtime_not_freed_after_fifty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="lr-notfree", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    for _ in range(50):
        run = await agent.run()
        await run.drain_events()
    assert not rt.is_freed
    rt.free()


@pytest.mark.asyncio
async def test_long_run_unique_ids_across_fifty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="lr-uniq", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    ids = set()
    for _ in range(50):
        run = await agent.run()
        assert run.run_id not in ids
        ids.add(run.run_id)
        await run.drain_events()
    assert len(ids) == 50
    rt.free()


@pytest.mark.asyncio
async def test_long_run_hundred_runtimes():
    for i in range(100):
        rt = ancora.Runtime()
        assert not rt.is_freed
        rt.free()
        assert rt.is_freed


@pytest.mark.asyncio
async def test_long_run_memory_store_five_hundred_ops():
    from ancora.memory import MemoryStore
    mem = MemoryStore()
    for i in range(500):
        mem.write(f"key-{i}", i * 2)
    for i in range(500):
        assert mem.read(f"key-{i}") == i * 2


@pytest.mark.asyncio
async def test_long_run_ten_concurrent_burst():
    rt = ancora.Runtime()
    spec = AgentSpec(name="lr-burst", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = await asyncio.gather(*[agent.run() for _ in range(10)])
    ids = {r.run_id for r in runs}
    assert len(ids) == 10
    rt.free()


@pytest.mark.asyncio
async def test_long_run_drain_all_events_non_zero():
    rt = ancora.Runtime()
    spec = AgentSpec(name="lr-drain", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    for _ in range(10):
        run = await agent.run()
        events = await run.drain_events()
        assert len(events) > 0
    rt.free()
