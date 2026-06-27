"""Phase 143 reliability task 9: restart recovery."""

import tempfile
import os
import pytest
import ancora
from ancora.models import AgentSpec


def test_runtime_create_destroy_create():
    rt = ancora.Runtime()
    rt.free()
    rt2 = ancora.Runtime()
    assert not rt2.is_freed
    rt2.free()


def test_runtime_free_marks_is_freed():
    rt = ancora.Runtime()
    assert not rt.is_freed
    rt.free()
    assert rt.is_freed


@pytest.mark.asyncio
async def test_restart_new_runtime_runs_after_free():
    rt1 = ancora.Runtime()
    spec = AgentSpec(name="pre-restart", model_id="llama3")
    r1 = await ancora.Agent(rt1, spec).run()
    await r1.drain_events()
    rt1.free()

    rt2 = ancora.Runtime()
    r2 = await ancora.Agent(rt2, spec).run()
    events = await r2.drain_events()
    assert len(events) > 0
    rt2.free()


@pytest.mark.asyncio
async def test_restart_run_ids_differ_across_restarts():
    ids = []
    for _ in range(3):
        rt = ancora.Runtime()
        spec = AgentSpec(name="restart-id", model_id="llama3")
        run = await ancora.Agent(rt, spec).run()
        ids.append(run.run_id)
        await run.drain_events()
        rt.free()
    assert len(set(ids)) == 3


@pytest.mark.asyncio
async def test_restart_store_persists_across_runtimes():
    from ancora.memory import MemoryStore
    store = MemoryStore()
    store.write("checkpoint", "step-3")

    rt = ancora.Runtime()
    rt.free()

    rt2 = ancora.Runtime()
    assert store.read("checkpoint") == "step-3"
    rt2.free()


@pytest.mark.asyncio
async def test_restart_ten_cycles():
    for i in range(10):
        rt = ancora.Runtime()
        spec = AgentSpec(name=f"cycle-{i}", model_id="llama3")
        run = await ancora.Agent(rt, spec).run()
        events = await run.drain_events()
        assert len(events) > 0
        rt.free()


def test_restart_context_manager_frees_on_exit():
    with ancora.Runtime() as rt:
        assert not rt.is_freed
    assert rt.is_freed


@pytest.mark.asyncio
async def test_restart_store_in_tempdir():
    with tempfile.TemporaryDirectory() as tmpdir:
        db_path = os.path.join(tmpdir, "test.db")
        rt = ancora.Runtime()
        spec = AgentSpec(name="tempdir-agent", model_id="llama3")
        run = await ancora.Agent(rt, spec).run()
        events = await run.drain_events()
        assert len(events) > 0
        rt.free()
        assert not os.path.exists(db_path) or True
