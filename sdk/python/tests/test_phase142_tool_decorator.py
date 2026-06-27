"""Phase 142 task 4: tool decorator execution."""

import pytest
import ancora
from ancora.tools import tool, Tool, ToolRegistry


def test_tool_decorator_creates_tool():
    @tool
    def my_tool(query: str) -> str:
        return f"result for {query}"

    assert isinstance(my_tool, Tool)


def test_tool_decorator_name_matches_function():
    @tool
    def search_web(query: str) -> str:
        return "result"

    assert my_tool_name(search_web) == "search_web"


def my_tool_name(t: Tool) -> str:
    return t.name


def test_tool_decorator_call_with_json():
    @tool
    def echo_tool(text: str) -> str:
        return text

    result = echo_tool.call_with_json('{"text": "hello"}')
    assert result == "hello"


def test_tool_decorator_call_with_kwargs():
    @tool
    def add_tool(a: int, b: int) -> int:
        return a + b

    result = add_tool.call_with_kwargs(a=2, b=3)
    assert result == 5


def test_tool_registry_register_and_get():
    @tool
    def reg_tool(x: str) -> str:
        return x

    reg = ToolRegistry()
    reg.register(reg_tool)
    assert reg.get("reg_tool") is not None


def test_tool_registry_dispatch():
    @tool
    def dispatch_tool(value: str) -> str:
        return value.upper()

    reg = ToolRegistry()
    reg.register(dispatch_tool)
    result = reg.dispatch("dispatch_tool", '{"value": "hello"}')
    assert result == "HELLO"


def test_tool_registry_dispatch_unknown_raises():
    reg = ToolRegistry()
    with pytest.raises(KeyError):
        reg.dispatch("not_registered", "{}")


def test_tool_registry_all_specs():
    @tool
    def spec_tool(x: int) -> int:
        return x

    reg = ToolRegistry()
    reg.register(spec_tool)
    specs = reg.all_specs()
    assert len(specs) == 1
    assert specs[0].name == "spec_tool"


def test_tool_spec_is_populated():
    @tool
    def greet(name: str) -> str:
        return f"Hello, {name}"

    assert greet.spec.name == "greet"
    assert greet.spec.description != "" or greet.spec.input_schema_json != ""


def test_tool_callable_directly():
    @tool
    def double(x: int) -> int:
        return x * 2

    result = double(5)
    assert result == 10
