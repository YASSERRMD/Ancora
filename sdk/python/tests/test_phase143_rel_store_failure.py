"""Phase 143 reliability task 11: store failure recovery."""

import pytest
from ancora.memory import MemoryStore
import ancora
from ancora.models import AgentSpec


def test_store_write_after_clear_works():
    mem = MemoryStore()
    mem.write("k", "v")
    mem.clear()
    mem.write("k2", "v2")
    assert mem.read("k2") == "v2"
    assert mem.read("k") is None


def test_store_read_missing_does_not_raise():
    mem = MemoryStore()
    result = mem.read("does-not-exist")
    assert result is None


def test_store_delete_nonexistent_does_not_raise():
    mem = MemoryStore()
    mem.delete("phantom")


def test_store_overwrite_value_works():
    mem = MemoryStore()
    mem.write("x", 1)
    mem.write("x", 2)
    assert mem.read("x") == 2


def test_store_recovery_after_partial_update():
    mem = MemoryStore()
    mem.update({"a": 1, "b": 2})
    mem.delete("a")
    assert mem.read("a") is None
    assert mem.read("b") == 2


def test_store_pop_on_missing_returns_default():
    mem = MemoryStore()
    val = mem.pop("missing", default="safe")
    assert val == "safe"


@pytest.mark.asyncio
async def test_store_failure_agent_still_starts():
    mem = MemoryStore()
    mem.clear()
    rt = ancora.Runtime()
    spec = AgentSpec(name="post-store-fail", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    await run.drain_events()
    rt.free()


@pytest.mark.asyncio
async def test_store_failure_multiple_recoveries():
    rt = ancora.Runtime()
    spec = AgentSpec(name="recover-multi", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    for _ in range(3):
        mem = MemoryStore()
        mem.clear()
        run = await agent.run()
        events = await run.drain_events()
        assert len(events) > 0
    rt.free()


def test_store_large_value_stored_and_retrieved():
    mem = MemoryStore()
    big = "x" * 100_000
    mem.write("large", big)
    result = mem.read("large")
    assert len(result) == 100_000
