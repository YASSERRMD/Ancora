"""Phase 143 e2e task 5: mcp end to end."""

import json
import pytest
import ancora
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec, ToolSpec


MCP_FIXTURE = {
    "server": "test-mcp-server",
    "tools": ["read_file", "write_file", "list_dir"],
    "auth_token": "fixture-token-000",
}


@tool
def mcp_read_file(path: str) -> str:
    """Read a file via the MCP fixture."""
    return json.dumps({"path": path, "content": "fixture content", "size": 14})


@tool
def mcp_list_dir(path: str) -> str:
    """List directory via the MCP fixture."""
    return json.dumps({"path": path, "entries": ["a.txt", "b.txt"]})


def test_mcp_fixture_has_server_name():
    assert MCP_FIXTURE["server"] == "test-mcp-server"


def test_mcp_fixture_tools_count():
    assert len(MCP_FIXTURE["tools"]) == 3


def test_mcp_read_file_returns_json():
    result = mcp_read_file.call_with_json('{"path": "/etc/hosts"}')
    parsed = json.loads(result)
    assert "content" in parsed


def test_mcp_list_dir_returns_entries():
    result = mcp_list_dir.call_with_json('{"path": "/tmp"}')
    parsed = json.loads(result)
    assert "entries" in parsed
    assert len(parsed["entries"]) == 2


def test_mcp_tools_registered():
    reg = ToolRegistry()
    reg.register(mcp_read_file)
    reg.register(mcp_list_dir)
    assert reg.get("mcp_read_file") is not None
    assert reg.get("mcp_list_dir") is not None


def test_mcp_dispatch_read_file():
    reg = ToolRegistry()
    reg.register(mcp_read_file)
    result = reg.dispatch("mcp_read_file", '{"path": "/tmp/test.txt"}')
    assert "content" in result


@pytest.mark.asyncio
async def test_mcp_agent_run_with_mcp_tools():
    rt = ancora.Runtime()
    tools = [
        ToolSpec(name="mcp-read-file", description="Read file via MCP"),
        ToolSpec(name="mcp-list-dir", description="List directory via MCP"),
    ]
    spec = AgentSpec(name="mcp-e2e", model_id="llama3", tools=tools)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert run.run_id != ""
    assert len(events) > 0
    rt.free()


@pytest.mark.asyncio
async def test_mcp_auth_token_not_leaked_in_events():
    rt = ancora.Runtime()
    spec = AgentSpec(name="mcp-auth", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    for ev in events:
        assert MCP_FIXTURE["auth_token"] not in ev.decode("utf-8", errors="replace")
    rt.free()


@pytest.mark.asyncio
async def test_mcp_two_tool_calls_in_sequence():
    rt = ancora.Runtime()
    reg = ToolRegistry()
    reg.register(mcp_read_file)
    reg.register(mcp_list_dir)
    result1 = reg.dispatch("mcp_read_file", '{"path": "/etc/hosts"}')
    result2 = reg.dispatch("mcp_list_dir", '{"path": "/etc"}')
    assert "content" in result1
    assert "entries" in result2
    rt.free()


@pytest.mark.asyncio
async def test_mcp_e2e_multiple_runs_distinct():
    rt = ancora.Runtime()
    spec = AgentSpec(name="mcp-multi", model_id="llama3")
    agent = ancora.Agent(rt, spec)
    runs = [await agent.run() for _ in range(3)]
    ids = {r.run_id for r in runs}
    assert len(ids) == 3
    rt.free()
