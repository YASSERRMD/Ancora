"""Phase 143 perf task 16: call overhead benchmark."""

import time
import pytest
import ancora
from ancora.models import AgentSpec
from ancora.tools import tool


@tool
def noop_tool(x: int) -> int:
    """No-op tool for overhead measurement."""
    return x


def bench_tool_call(n: int = 1000) -> float:
    start = time.monotonic()
    for i in range(n):
        noop_tool.call_with_json(f'{{"x": {i}}}')
    return time.monotonic() - start


def test_tool_call_overhead_thousand_ops_under_five_seconds():
    elapsed = bench_tool_call(1000)
    assert elapsed < 5.0, f"1000 tool calls took {elapsed:.3f}s"


def test_tool_call_overhead_hundred_ops_under_one_second():
    elapsed = bench_tool_call(100)
    assert elapsed < 1.0, f"100 tool calls took {elapsed:.3f}s"


def test_runtime_create_overhead_ten_times_under_five_seconds():
    start = time.monotonic()
    rts = []
    for _ in range(10):
        rts.append(ancora.Runtime())
    elapsed = time.monotonic() - start
    assert elapsed < 5.0, f"10 Runtime() creations took {elapsed:.3f}s"
    for rt in rts:
        rt.free()


def test_runtime_free_overhead_ten_times_under_one_second():
    rts = [ancora.Runtime() for _ in range(10)]
    start = time.monotonic()
    for rt in rts:
        rt.free()
    elapsed = time.monotonic() - start
    assert elapsed < 1.0, f"10 Runtime.free() calls took {elapsed:.3f}s"


@pytest.mark.asyncio
async def test_run_start_overhead_ten_runs_under_five_seconds():
    rt = ancora.Runtime()
    spec = AgentSpec(name="perf-run", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    start = time.monotonic()
    for _ in range(10):
        run = await agent.run()
        await run.drain_events()
    elapsed = time.monotonic() - start
    assert elapsed < 5.0, f"10 runs took {elapsed:.3f}s"
    rt.free()


def test_tool_dispatch_throughput_acceptable():
    from ancora.tools import ToolRegistry
    reg = ToolRegistry()
    reg.register(noop_tool)
    start = time.monotonic()
    for i in range(500):
        reg.dispatch("noop_tool", f'{{"x": {i}}}')
    elapsed = time.monotonic() - start
    assert elapsed < 5.0, f"500 dispatches took {elapsed:.3f}s"


def test_tool_call_is_deterministic():
    r1 = noop_tool.call_with_json('{"x": 42}')
    r2 = noop_tool.call_with_json('{"x": 42}')
    assert r1 == r2
