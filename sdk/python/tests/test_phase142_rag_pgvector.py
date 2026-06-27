"""Phase 142 task 11: rag retrieval pgvector."""

import json
import pytest
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec, ToolSpec
import ancora


PGVECTOR_FIXTURE = [
    {"id": "doc-1", "text": "PostgreSQL is an object-relational database.", "score": 0.93},
    {"id": "doc-2", "text": "pgvector adds vector similarity search to Postgres.", "score": 0.88},
    {"id": "doc-3", "text": "Cosine similarity compares vector directions.", "score": 0.80},
]


@tool
def pgvector_retrieve(query: str, top_k: int = 3) -> str:
    """Retrieve relevant chunks from PgVector fixture."""
    return json.dumps(PGVECTOR_FIXTURE[:top_k])


def test_pgvector_retrieve_returns_json():
    result = pgvector_retrieve.call_with_json('{"query": "postgres", "top_k": 3}')
    parsed = json.loads(result)
    assert len(parsed) == 3


def test_pgvector_retrieve_chunks_have_ids():
    result = pgvector_retrieve.call_with_json('{"query": "vector", "top_k": 3}')
    chunks = json.loads(result)
    for chunk in chunks:
        assert chunk["id"] != ""


def test_pgvector_retrieve_chunks_have_text():
    result = pgvector_retrieve.call_with_json('{"query": "search", "top_k": 3}')
    chunks = json.loads(result)
    for chunk in chunks:
        assert chunk["text"] != ""


def test_pgvector_retrieve_scores_descend():
    result = pgvector_retrieve.call_with_json('{"query": "test", "top_k": 3}')
    chunks = json.loads(result)
    for i in range(1, len(chunks)):
        assert chunks[i]["score"] <= chunks[i - 1]["score"]


def test_pgvector_tool_registered_in_registry():
    reg = ToolRegistry()
    reg.register(pgvector_retrieve)
    assert reg.get("pgvector_retrieve") is not None


def test_pgvector_tool_invoked_via_registry():
    reg = ToolRegistry()
    reg.register(pgvector_retrieve)
    result = reg.dispatch("pgvector_retrieve", '{"query": "postgres", "top_k": 2}')
    chunks = json.loads(result)
    assert len(chunks) == 2


def test_pgvector_agent_spec_includes_retrieve_tool():
    ts = ToolSpec(name="pgvector-retrieve", description="retrieves chunks from PgVector")
    spec = AgentSpec(name="rag-agent", model_id="gpt-4o", tools=[ts])
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "pgvector-retrieve"


@pytest.mark.asyncio
async def test_pgvector_agent_run_starts():
    rt = ancora.Runtime()
    ts = ToolSpec(name="pgvector-retrieve", description="retrieves chunks from PgVector")
    spec = AgentSpec(name="rag-pgvec", model_id="llama3", tools=[ts])
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


def test_pgvector_fixture_has_three_chunks():
    assert len(PGVECTOR_FIXTURE) == 3


def test_pgvector_retrieve_top_k_limits_results():
    result = pgvector_retrieve.call_with_json('{"query": "test", "top_k": 1}')
    chunks = json.loads(result)
    assert len(chunks) == 1
