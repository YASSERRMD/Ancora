"""Phase 143 e2e task 4: rag with pgvector end to end."""

import json
import pytest
import ancora
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec, ToolSpec


PG_CHUNKS = [
    {"id": "pg-1", "text": "PostgreSQL supports JSONB for flexible schema.", "score": 0.95},
    {"id": "pg-2", "text": "pgvector enables ANN search on embeddings.", "score": 0.90},
    {"id": "pg-3", "text": "HNSW indexing improves recall at high QPS.", "score": 0.85},
    {"id": "pg-4", "text": "IVFFlat is faster to build than HNSW.", "score": 0.78},
]


@tool
def pg_retrieve(query: str, top_k: int = 4) -> str:
    """Retrieve chunks from the PgVector fixture."""
    return json.dumps(PG_CHUNKS[:top_k])


def test_pg_chunks_fixture_count():
    assert len(PG_CHUNKS) == 4


def test_pg_retrieve_returns_all_chunks_by_default():
    result = pg_retrieve.call_with_json('{"query": "embedding"}')
    chunks = json.loads(result)
    assert len(chunks) == 4


def test_pg_retrieve_top_k_respected():
    result = pg_retrieve.call_with_json('{"query": "indexing", "top_k": 2}')
    chunks = json.loads(result)
    assert len(chunks) == 2


def test_pg_chunks_have_ids():
    result = pg_retrieve.call_with_json('{"query": "ann"}')
    for c in json.loads(result):
        assert c["id"].startswith("pg-")


def test_pg_chunks_have_scores():
    result = pg_retrieve.call_with_json('{"query": "ann"}')
    for c in json.loads(result):
        assert 0.0 < c["score"] <= 1.0


def test_pg_chunks_scores_descending():
    result = pg_retrieve.call_with_json('{"query": "ann"}')
    scores = [c["score"] for c in json.loads(result)]
    assert scores == sorted(scores, reverse=True)


def test_pg_tool_in_registry():
    reg = ToolRegistry()
    reg.register(pg_retrieve)
    assert reg.get("pg_retrieve") is not None


def test_pg_tool_dispatch_via_registry():
    reg = ToolRegistry()
    reg.register(pg_retrieve)
    res = reg.dispatch("pg_retrieve", '{"query": "hnsw", "top_k": 2}')
    assert len(json.loads(res)) == 2


@pytest.mark.asyncio
async def test_pg_e2e_agent_run_with_retrieve_tool():
    rt = ancora.Runtime()
    ts = ToolSpec(name="pg-retrieve", description="Retrieve pgvector chunks")
    spec = AgentSpec(name="pg-rag-agent", model_id="llama3", tools=[ts])
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert run.run_id != ""
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_pg_rag_two_consecutive_queries():
    rt = ancora.Runtime()
    ts = ToolSpec(name="pg-retrieve", description="Retrieve pgvector chunks")
    spec = AgentSpec(name="pg-rag-seq", model_id="llama3", tools=[ts])
    agent = ancora.Agent(rt, spec)
    r1 = await agent.run()
    r2 = await agent.run()
    assert r1.run_id != r2.run_id
    rt.free()
