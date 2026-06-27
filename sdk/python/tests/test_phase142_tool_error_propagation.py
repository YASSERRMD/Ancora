"""Phase 142 task 5: tool error propagation."""

import pytest
from ancora.tools import tool, Tool, ToolRegistry


def test_tool_error_propagates_from_call_with_json():
    @tool
    def failing_tool(x: str) -> str:
        raise ValueError("tool-specific error")

    with pytest.raises(ValueError, match="tool-specific error"):
        failing_tool.call_with_json('{"x": "test"}')


def test_tool_error_propagates_from_dispatch():
    @tool
    def error_dispatch(msg: str) -> str:
        raise RuntimeError(msg)

    reg = ToolRegistry()
    reg.register(error_dispatch)
    with pytest.raises(RuntimeError, match="dispatch error"):
        reg.dispatch("error_dispatch", '{"msg": "dispatch error"}')


def test_tool_error_does_not_affect_other_tools():
    @tool
    def good_tool(x: str) -> str:
        return x

    @tool
    def bad_tool(x: str) -> str:
        raise ValueError("bad")

    reg = ToolRegistry()
    reg.register(good_tool)
    reg.register(bad_tool)

    result = reg.dispatch("good_tool", '{"x": "ok"}')
    assert result == "ok"

    with pytest.raises(ValueError):
        reg.dispatch("bad_tool", '{"x": "ignored"}')


def test_tool_error_class_is_preserved():
    class CustomError(Exception):
        pass

    @tool
    def custom_error_tool(x: str) -> str:
        raise CustomError("custom")

    reg = ToolRegistry()
    reg.register(custom_error_tool)

    with pytest.raises(CustomError):
        reg.dispatch("custom_error_tool", '{"x": "x"}')


def test_tool_registry_dispatch_unregistered_raises_keyerror():
    reg = ToolRegistry()
    with pytest.raises(KeyError):
        reg.dispatch("nonexistent", "{}")


def test_tool_call_with_invalid_json_raises():
    @tool
    def json_tool(x: str) -> str:
        return x

    with pytest.raises(Exception):
        json_tool.call_with_json("not valid json")


def test_tool_error_from_call_with_kwargs():
    @tool
    def typed_tool(n: int) -> int:
        if n < 0:
            raise ValueError("negative not allowed")
        return n

    with pytest.raises(ValueError, match="negative not allowed"):
        typed_tool.call_with_kwargs(n=-1)


def test_tool_registry_get_returns_none_for_unknown():
    reg = ToolRegistry()
    result = reg.get("unknown")
    assert result is None


def test_tool_error_message_is_preserved():
    @tool
    def msg_tool(x: str) -> str:
        raise ValueError(f"error for: {x}")

    reg = ToolRegistry()
    reg.register(msg_tool)
    with pytest.raises(ValueError, match="error for: test-value"):
        reg.dispatch("msg_tool", '{"x": "test-value"}')


def test_tool_registry_names_excludes_unregistered():
    reg = ToolRegistry()

    @tool
    def t1(x: str) -> str:
        return x

    reg.register(t1)
    assert "t1" in reg.names
    assert "nonexistent" not in reg.names
