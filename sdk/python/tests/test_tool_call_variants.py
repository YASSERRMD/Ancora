"""Tests for Tool call variants and ToolRegistry.names."""

from ancora.tools import Tool, ToolRegistry, tool


def test_call_with_kwargs():
    @tool
    def add(a: int, b: int) -> int:
        return a + b
    assert add.call_with_kwargs(a=3, b=4) == 7


def test_call_with_kwargs_single_param():
    @tool
    def double(n: int) -> int:
        return n * 2
    assert double.call_with_kwargs(n=5) == 10


def test_tool_registry_names_empty():
    reg = ToolRegistry()
    assert reg.names == []


def test_tool_registry_names_ordered():
    @tool(name="alpha")
    def t1() -> None: pass
    @tool(name="beta")
    def t2() -> None: pass
    @tool(name="gamma")
    def t3() -> None: pass

    reg = ToolRegistry()
    reg.register(t1)
    reg.register(t2)
    reg.register(t3)
    assert reg.names == ["alpha", "beta", "gamma"]


def test_call_with_json_empty_string():
    @tool
    def fn() -> str:
        return "ok"
    result = fn.call_with_json("")
    assert result == "ok"


def test_call_with_json_and_call_with_kwargs_equivalent():
    @tool
    def multiply(x: int, y: int) -> int:
        return x * y
    assert multiply.call_with_json('{"x": 3, "y": 4}') == multiply.call_with_kwargs(x=3, y=4)
