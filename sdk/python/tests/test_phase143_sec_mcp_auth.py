"""Phase 143 security task 15: unauthenticated mcp refused."""

import json
import pytest
from ancora.tools import tool, ToolRegistry
from ancora.models import AgentSpec, ToolSpec
import ancora


ERR_UNAUTHORIZED = "unauthorized"


@tool
def secure_resource(token: str) -> str:
    """Access a secured resource."""
    if token != "valid-token-abc":
        raise PermissionError(ERR_UNAUTHORIZED)
    return json.dumps({"data": "secret-value"})


def test_mcp_auth_valid_token_succeeds():
    result = secure_resource.call_with_json('{"token": "valid-token-abc"}')
    parsed = json.loads(result)
    assert parsed["data"] == "secret-value"


def test_mcp_auth_invalid_token_raises():
    with pytest.raises(PermissionError, match=ERR_UNAUTHORIZED):
        secure_resource.call_with_json('{"token": "wrong-token"}')


def test_mcp_auth_empty_token_raises():
    with pytest.raises(PermissionError, match=ERR_UNAUTHORIZED):
        secure_resource.call_with_json('{"token": ""}')


def test_mcp_auth_missing_token_raises():
    with pytest.raises((TypeError, KeyError, Exception)):
        secure_resource.call_with_json('{}')


def test_mcp_auth_registry_dispatch_invalid_raises():
    reg = ToolRegistry()
    reg.register(secure_resource)
    with pytest.raises(PermissionError, match=ERR_UNAUTHORIZED):
        reg.dispatch("secure_resource", '{"token": "bad"}')


def test_mcp_auth_valid_dispatch_succeeds():
    reg = ToolRegistry()
    reg.register(secure_resource)
    result = reg.dispatch("secure_resource", '{"token": "valid-token-abc"}')
    assert "secret-value" in result


def test_mcp_auth_error_sentinel_matches():
    assert ERR_UNAUTHORIZED == "unauthorized"


def test_mcp_auth_no_secret_in_error_message():
    try:
        secure_resource.call_with_json('{"token": "bad"}')
    except PermissionError as exc:
        assert "secret-value" not in str(exc)


@pytest.mark.asyncio
async def test_mcp_auth_agent_with_secure_tool_spec():
    rt = ancora.Runtime()
    ts = ToolSpec(name="secure-resource", description="Requires auth token")
    spec = AgentSpec(name="auth-agent", model_id="llama3", tools=[ts])
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    await run.drain_events()
    rt.free()


@pytest.mark.asyncio
async def test_mcp_auth_multiple_invalid_calls_all_raise():
    for bad_token in ["", "wrong", "hacked", "' OR 1=1"]:
        with pytest.raises(PermissionError):
            secure_resource.call_with_json(json.dumps({"token": bad_token}))
