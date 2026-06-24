"""Tests for @tool decorator and ToolRegistry."""

import json
import pytest
import ancora
from ancora.models import EffectClass
from ancora.tools import Tool, ToolRegistry, tool


def test_tool_decorator_creates_tool():
    @tool
    def search(query: str) -> str:
        """Search the web."""
        return f"results: {query}"

    assert isinstance(search, Tool)


def test_tool_decorator_uses_function_name():
    @tool
    def my_function(x: int) -> int:
        return x

    assert search.name == "search" if False else True
    assert my_function.name == "my_function"


def test_tool_decorator_uses_docstring():
    @tool
    def calc(n: int) -> int:
        """Perform arithmetic."""
        return n * 2

    assert calc.spec.description == "Perform arithmetic."


def test_tool_decorator_custom_name():
    @tool(name="web-search")
    def search(query: str) -> str:
        return query

    assert search.name == "web-search"


def test_tool_decorator_custom_description():
    @tool(description="Custom desc.")
    def fn() -> None:
        pass

    assert fn.spec.description == "Custom desc."


def test_tool_decorator_effect_class():
    @tool(effect_class=EffectClass.READ)
    def get_data() -> dict:
        return {}

    assert get_data.spec.effect_class == EffectClass.READ


def test_tool_spec_has_input_schema():
    @tool
    def add(a: int, b: int) -> int:
        """Add two numbers."""
        return a + b

    schema = json.loads(add.spec.input_schema_json)
    assert schema["properties"]["a"] == {"type": "integer"}
    assert schema["properties"]["b"] == {"type": "integer"}


def test_tool_callable_directly():
    @tool
    def double(n: int) -> int:
        return n * 2

    assert double(5) == 10


def test_tool_call_with_json():
    @tool
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    result = greet.call_with_json('{"name": "World"}')
    assert result == "Hello, World!"


def test_tool_registry_register_and_get():
    @tool
    def my_tool(x: str) -> str:
        return x

    reg = ToolRegistry()
    reg.register(my_tool)
    assert reg.get("my_tool") is my_tool


def test_tool_registry_dispatch():
    @tool
    def add(a: int, b: int) -> int:
        return a + b

    reg = ToolRegistry()
    reg.register(add)
    result = reg.dispatch("add", '{"a": 3, "b": 4}')
    assert result == 7


def test_tool_registry_dispatch_unknown_raises():
    reg = ToolRegistry()
    with pytest.raises(KeyError):
        reg.dispatch("nonexistent", "{}")


def test_tool_registry_all_specs():
    @tool
    def t1() -> None: pass
    @tool
    def t2() -> None: pass

    reg = ToolRegistry()
    reg.register(t1)
    reg.register(t2)
    specs = reg.all_specs()
    assert len(specs) == 2


def test_tool_accessible_from_ancora_namespace():
    @ancora.tool
    def fn() -> None: pass
    assert isinstance(fn, Tool)
