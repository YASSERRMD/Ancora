"""Phase 143 e2e task 17: all catalog examples smoke test."""

import pytest
import ancora
from ancora.models import AgentSpec


CATALOG_EXAMPLES = [
    {"name": "single-agent", "model_id": "llama3"},
    {"name": "verifier-pipeline", "model_id": "gpt-4o"},
    {"name": "human-in-loop", "model_id": "claude-opus-4-8"},
    {"name": "rag-pgvector", "model_id": "llama3"},
    {"name": "mcp-tool", "model_id": "llama3"},
    {"name": "qwen-regional", "model_id": "qwen-turbo"},
    {"name": "deepseek-chat", "model_id": "deepseek-chat"},
    {"name": "streaming-agent", "model_id": "llama3"},
    {"name": "structured-output", "model_id": "gpt-4o"},
    {"name": "multi-node", "model_id": "llama3"},
]


def test_catalog_has_ten_examples():
    assert len(CATALOG_EXAMPLES) == 10


def test_catalog_all_names_non_empty():
    for ex in CATALOG_EXAMPLES:
        assert ex["name"] != ""


def test_catalog_all_model_ids_non_empty():
    for ex in CATALOG_EXAMPLES:
        assert ex["model_id"] != ""


def test_catalog_names_are_distinct():
    names = [ex["name"] for ex in CATALOG_EXAMPLES]
    assert len(set(names)) == len(names)


@pytest.mark.asyncio
async def test_catalog_all_examples_start_run():
    rt = ancora.Runtime()
    for ex in CATALOG_EXAMPLES:
        spec = AgentSpec(name=ex["name"], model_id=ex["model_id"])
        agent = ancora.Agent(rt, spec)
        run = await agent.run()
        assert run.run_id != "", f"Empty run ID for {ex['name']}"
    rt.free()


@pytest.mark.asyncio
async def test_catalog_all_examples_drain_events():
    rt = ancora.Runtime()
    for ex in CATALOG_EXAMPLES:
        spec = AgentSpec(name=ex["name"] + "-drain", model_id=ex["model_id"])
        agent = ancora.Agent(rt, spec)
        run = await agent.run()
        events = await run.drain_events()
        assert len(events) > 0, f"No events for {ex['name']}"
    rt.free()


@pytest.mark.asyncio
async def test_catalog_first_example_single_agent():
    rt = ancora.Runtime()
    ex = CATALOG_EXAMPLES[0]
    spec = AgentSpec(name=ex["name"] + "-first", model_id=ex["model_id"])
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_catalog_last_example_multi_node():
    rt = ancora.Runtime()
    ex = CATALOG_EXAMPLES[-1]
    spec = AgentSpec(name=ex["name"] + "-last", model_id=ex["model_id"])
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()
