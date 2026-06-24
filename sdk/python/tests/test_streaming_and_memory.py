"""Integration test: streaming and memory work together in a run."""

import asyncio
import json

import ancora
from ancora.memory import MemoryStore
from ancora.models import StreamEvent


async def test_streaming_writes_tokens_to_memory():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    mem = MemoryStore()
    agent = ancora.Agent(rt, spec, memory=mem)
    run = await agent.run()

    tokens = []
    async for raw in run.stream_events():
        ev = StreamEvent.from_bytes(raw)
        if ev.is_token and ev.text:
            tokens.append(ev.text)
            agent.memory.write(f"token_{len(tokens)}", ev.text)

    assert agent.memory.read("token_1") == "Hello"
    assert agent.memory.read("token_2") == " "
    assert agent.memory.read("token_3") == "world"
    rt.free()


async def test_memory_survives_multiple_runs():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    mem = MemoryStore()
    agent = ancora.Agent(rt, spec, memory=mem)

    run1 = await agent.run()
    await run1.drain_events()
    mem.write("run_count", 1)

    run2 = await agent.run()
    await run2.drain_events()
    count = mem.read("run_count", default=0) + 1
    mem.write("run_count", count)

    assert mem.read("run_count") == 2
    rt.free()


async def test_stream_events_total_count():
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(name="a", model_id="m")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()

    events = [json.loads(raw) async for raw in run.stream_events()]
    assert len(events) == 5

    kinds = [e["kind"] for e in events]
    assert kinds.count("token") == 3
    assert kinds.count("started") == 1
    assert kinds.count("completed") == 1

    rt.free()
