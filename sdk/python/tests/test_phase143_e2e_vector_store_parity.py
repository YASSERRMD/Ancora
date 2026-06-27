"""Phase 143 e2e task 18: two vector stores parity."""

import json
import pytest
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec, ToolSpec
import ancora


LANCEDB_FIXTURE = [
    {"id": "lance-1", "text": "LanceDB stores embeddings in Lance format.", "score": 0.94},
    {"id": "lance-2", "text": "Lance columnar format enables fast random access.", "score": 0.87},
]

PGVECTOR_FIXTURE2 = [
    {"id": "pgv-1", "text": "pgvector supports cosine and L2 distance.", "score": 0.92},
    {"id": "pgv-2", "text": "pgvector integrates natively with PostgreSQL.", "score": 0.85},
]


@tool
def lance_retrieve2(query: str, top_k: int = 2) -> str:
    """Retrieve from LanceDB fixture."""
    return json.dumps(LANCEDB_FIXTURE[:top_k])


@tool
def pg_retrieve2(query: str, top_k: int = 2) -> str:
    """Retrieve from PgVector fixture."""
    return json.dumps(PGVECTOR_FIXTURE2[:top_k])


def test_both_fixtures_same_schema():
    for chunk in LANCEDB_FIXTURE + PGVECTOR_FIXTURE2:
        assert "id" in chunk
        assert "text" in chunk
        assert "score" in chunk


def test_lance_fixture_count():
    assert len(LANCEDB_FIXTURE) == 2


def test_pgvector2_fixture_count():
    assert len(PGVECTOR_FIXTURE2) == 2


def test_lance_ids_distinct_from_pg_ids():
    lance_ids = {c["id"] for c in LANCEDB_FIXTURE}
    pg_ids = {c["id"] for c in PGVECTOR_FIXTURE2}
    assert lance_ids.isdisjoint(pg_ids)


def test_lance_retrieve2_returns_chunks():
    result = lance_retrieve2.call_with_json('{"query": "embedding"}')
    chunks = json.loads(result)
    assert len(chunks) == 2


def test_pg_retrieve2_returns_chunks():
    result = pg_retrieve2.call_with_json('{"query": "distance"}')
    chunks = json.loads(result)
    assert len(chunks) == 2


def test_both_tools_in_registry():
    reg = ToolRegistry()
    reg.register(lance_retrieve2)
    reg.register(pg_retrieve2)
    assert reg.get("lance_retrieve2") is not None
    assert reg.get("pg_retrieve2") is not None


def test_parity_scores_both_non_zero():
    for chunk in LANCEDB_FIXTURE + PGVECTOR_FIXTURE2:
        assert chunk["score"] > 0.0


@pytest.mark.asyncio
async def test_parity_agent_with_both_vector_tools():
    rt = ancora.Runtime()
    tools = [
        ToolSpec(name="lance-retrieve", description="LanceDB retrieval"),
        ToolSpec(name="pg-retrieve", description="PgVector retrieval"),
    ]
    spec = AgentSpec(name="dual-vec-agent", model_id="llama3", tools=tools)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert run.run_id != ""
    assert len(events) > 0
    rt.free()
