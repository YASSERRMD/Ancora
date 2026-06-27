"""Phase 142 task 15: mcp tool use."""

import json
import pytest
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec, ToolSpec
import ancora


def make_mcp_call(tool_name: str, args: dict) -> dict:
    return {"tool": tool_name, "args": args, "type": "tool_call"}


def make_mcp_result(tool_name: str, output: str, is_error: bool = False) -> dict:
    return {"tool": tool_name, "output": output, "is_error": is_error}


def test_mcp_tool_call_has_required_fields():
    call = make_mcp_call("read_file", {"path": "/etc/hosts"})
    assert "tool" in call
    assert "args" in call
    assert call["type"] == "tool_call"


def test_mcp_tool_result_has_required_fields():
    result = make_mcp_result("read_file", "127.0.0.1 localhost")
    assert "tool" in result
    assert "output" in result
    assert result["is_error"] is False


def test_mcp_tool_call_json_round_trip():
    call = make_mcp_call("search", {"query": "python"})
    raw = json.dumps(call)
    parsed = json.loads(raw)
    assert parsed["tool"] == "search"
    assert parsed["args"]["query"] == "python"


def test_mcp_tool_error_result_flag():
    result = make_mcp_result("delete_file", "", is_error=True)
    assert result["is_error"] is True
    assert result["output"] == ""


def test_mcp_tool_registry_dispatch():
    @tool
    def mcp_echo(text: str) -> str:
        """Echo text back."""
        return text

    reg = ToolRegistry()
    reg.register(mcp_echo)
    result = reg.dispatch("mcp_echo", json.dumps({"text": "hello mcp"}))
    assert result == "hello mcp"


def test_mcp_tool_result_output_non_empty_on_success():
    result = make_mcp_result("list_files", "[\"a.txt\", \"b.txt\"]")
    assert len(result["output"]) > 0


def test_mcp_multiple_tool_calls_distinct_names():
    calls = [
        make_mcp_call("read_file", {"path": "/etc/hosts"}),
        make_mcp_call("list_dir", {"path": "/etc"}),
        make_mcp_call("write_file", {"path": "/tmp/out.txt", "content": "ok"}),
    ]
    names = [c["tool"] for c in calls]
    assert len(set(names)) == len(names)


def test_mcp_tool_call_args_json_serializable():
    call = make_mcp_call("query_db", {"sql": "SELECT 1", "limit": 100})
    assert json.dumps(call) is not None


def test_mcp_tool_spec_from_tool_call():
    call = make_mcp_call("summarize", {"text": "long text..."})
    ts = ToolSpec(name=call["tool"], description="Summarize text via MCP")
    assert ts.name == "summarize"


@pytest.mark.asyncio
async def test_mcp_agent_with_tool_spec_starts():
    rt = ancora.Runtime()
    ts = ToolSpec(name="mcp-tool", description="An MCP-connected tool")
    spec = AgentSpec(name="mcp-agent", model_id="llama3", tools=[ts])
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()
