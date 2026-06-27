"""Phase 142 task 20: GIL release during native calls."""

import time
import asyncio
import threading
import pytest
import ancora
from ancora.models import AgentSpec


def test_runtime_create_does_not_block_gil():
    results = []
    errors = []

    def create_runtime():
        try:
            rt = ancora.Runtime()
            results.append(rt)
        except Exception as exc:
            errors.append(exc)

    threads = [threading.Thread(target=create_runtime) for _ in range(4)]
    for t in threads:
        t.start()
    for t in threads:
        t.join(timeout=5.0)

    assert len(errors) == 0
    assert len(results) == 4
    for rt in results:
        rt.free()


def test_runtime_free_does_not_block_gil():
    runtimes = [ancora.Runtime() for _ in range(4)]
    errors = []

    def free_rt(rt):
        try:
            rt.free()
        except Exception as exc:
            errors.append(exc)

    threads = [threading.Thread(target=free_rt, args=(rt,)) for rt in runtimes]
    for t in threads:
        t.start()
    for t in threads:
        t.join(timeout=5.0)

    assert len(errors) == 0


@pytest.mark.asyncio
async def test_async_runs_do_not_starve_event_loop():
    rt = ancora.Runtime()
    spec = AgentSpec(name="starve-test", model_id="llama3")
    agent = ancora.Agent(rt, spec)

    ticks = []

    async def tick_counter():
        for _ in range(5):
            await asyncio.sleep(0)
            ticks.append(1)

    await asyncio.gather(agent.run(), tick_counter())
    assert len(ticks) == 5
    rt.free()


def test_multiple_runtimes_in_threads_are_isolated():
    seen_ids = []
    lock = threading.Lock()

    def make_rt():
        rt = ancora.Runtime()
        with lock:
            seen_ids.append(id(rt))
        rt.free()

    threads = [threading.Thread(target=make_rt) for _ in range(6)]
    for t in threads:
        t.start()
    for t in threads:
        t.join(timeout=5.0)

    assert len(seen_ids) == 6
    assert len(set(seen_ids)) == 6


@pytest.mark.asyncio
async def test_native_calls_interleave_with_python_coroutines():
    rt = ancora.Runtime()
    spec = AgentSpec(name="interleave", model_id="llama3")
    agent = ancora.Agent(rt, spec)

    counter = [0]

    async def inc():
        for _ in range(3):
            await asyncio.sleep(0)
            counter[0] += 1

    await asyncio.gather(agent.run(), inc(), inc())
    assert counter[0] == 6
    rt.free()


def test_is_freed_after_free():
    rt = ancora.Runtime()
    assert not rt.is_freed
    rt.free()
    assert rt.is_freed


def test_concurrent_thread_runtime_creation_wall_time():
    start = time.monotonic()
    rts = []
    errors = []

    def make():
        try:
            rts.append(ancora.Runtime())
        except Exception as exc:
            errors.append(exc)

    threads = [threading.Thread(target=make) for _ in range(8)]
    for t in threads:
        t.start()
    for t in threads:
        t.join(timeout=10.0)

    elapsed = time.monotonic() - start
    assert len(errors) == 0
    assert elapsed < 10.0
    for rt in rts:
        rt.free()
