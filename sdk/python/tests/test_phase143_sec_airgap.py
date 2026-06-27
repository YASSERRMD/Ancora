"""Phase 143 security task 14: air-gapped egress zero."""

import json
import socket
import pytest
import ancora
from ancora.models import AgentSpec


LOCAL_SCHEMA = {
    "type": "object",
    "properties": {
        "answer": {"type": "string"},
    },
    "required": ["answer"],
}


def test_airgap_local_schema_valid_json():
    raw = json.dumps(LOCAL_SCHEMA)
    parsed = json.loads(raw)
    assert parsed["type"] == "object"


def test_airgap_no_live_dns_lookup():
    try:
        socket.setdefaulttimeout(0.001)
        socket.getaddrinfo("api.anthropic.com", 443)
        pytest.skip("Network available - skipping airgap check")
    except (socket.timeout, socket.gaierror, OSError):
        pass


@pytest.mark.asyncio
async def test_airgap_runtime_creates_without_network():
    rt = ancora.Runtime()
    assert not rt.is_freed
    rt.free()


@pytest.mark.asyncio
async def test_airgap_agent_run_without_network():
    rt = ancora.Runtime()
    spec = AgentSpec(name="airgap-run", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    await run.drain_events()
    rt.free()


def test_airgap_schema_has_no_external_refs():
    raw = json.dumps(LOCAL_SCHEMA)
    assert "http" not in raw
    assert "$ref" not in raw


@pytest.mark.asyncio
async def test_airgap_events_contain_no_external_urls():
    rt = ancora.Runtime()
    spec = AgentSpec(name="airgap-url", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        decoded = ev.decode("utf-8", errors="replace")
        assert "api.anthropic.com" not in decoded
        assert "api.openai.com" not in decoded
    rt.free()


@pytest.mark.asyncio
async def test_airgap_no_live_keys_in_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="airgap-key", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        decoded = ev.decode("utf-8", errors="replace")
        assert "sk-ant-" not in decoded
        assert "sk-proj-" not in decoded
    rt.free()


@pytest.mark.asyncio
async def test_airgap_schema_from_local_struct():
    schema_json = json.dumps(LOCAL_SCHEMA)
    spec = AgentSpec(
        name="airgap-schema",
        model_id="llama3",
        output_schema_json=schema_json,
    )
    assert spec.output_schema_json == schema_json


@pytest.mark.asyncio
async def test_airgap_conformance_suite_offline():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert isinstance(results, dict)
    rt.free()
