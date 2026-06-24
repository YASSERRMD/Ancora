"""Edge case tests for tool decorator behavior."""

import pytest
from ancora.tools import Tool, ToolRegistry, tool
from ancora.models import EffectClass


def test_tool_repr():
    @tool
    def my_tool() -> None: pass
    assert "my_tool" in repr(my_tool)


def test_tool_without_docstring_empty_description():
    @tool
    def fn() -> None: pass
    assert fn.spec.description == ""


def test_tool_preserves_function_name():
    @tool
    def original_name() -> None: pass
    assert original_name.__name__ == "original_name"


def test_tool_preserves_docstring():
    @tool
    def fn() -> None:
        """My docstring."""
    assert fn.__doc__ == "My docstring."


def test_tool_is_callable():
    @tool
    def add(a: int, b: int) -> int:
        return a + b
    assert callable(add)
    assert add(1, 2) == 3


def test_tool_registry_len():
    reg = ToolRegistry()

    @tool
    def t1() -> None: pass
    @tool
    def t2() -> None: pass

    reg.register(t1)
    assert len(reg) == 1
    reg.register(t2)
    assert len(reg) == 2


def test_tool_registry_contains():
    @tool
    def my_tool() -> None: pass
    reg = ToolRegistry()
    assert "my_tool" not in reg
    reg.register(my_tool)
    assert "my_tool" in reg


def test_tool_registry_overwrite_on_duplicate():
    calls = []

    @tool(name="fn")
    def fn_v1() -> None:
        calls.append("v1")

    @tool(name="fn")
    def fn_v2() -> None:
        calls.append("v2")

    reg = ToolRegistry()
    reg.register(fn_v1)
    reg.register(fn_v2)
    reg.dispatch("fn", "{}")
    assert calls == ["v2"]


def test_tool_spec_effect_class_default():
    @tool
    def fn() -> None: pass
    assert fn.spec.effect_class == EffectClass.UNSPECIFIED


def test_tool_name_property():
    @tool(name="custom")
    def fn() -> None: pass
    assert fn.name == "custom"
