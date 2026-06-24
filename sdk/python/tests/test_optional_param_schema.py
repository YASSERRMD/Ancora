"""Tests for Optional parameter handling in JSON schema generation."""

import json
from typing import Optional

from ancora.schema import type_to_json_schema
from ancora.tools import tool


def test_optional_str_schema():
    schema = type_to_json_schema(Optional[str])
    assert schema == {"type": "string"}


def test_optional_int_schema():
    schema = type_to_json_schema(Optional[int])
    assert schema == {"type": "integer"}


def test_optional_param_not_required():
    @tool
    def greet(name: str, suffix: Optional[str] = None) -> str:
        return f"{name}{suffix or ''}"

    schema = json.loads(greet.spec.input_schema_json)
    assert "name" in schema.get("required", [])
    assert "suffix" not in schema.get("required", [])


def test_optional_param_present_in_properties():
    @tool
    def fn(a: int, b: Optional[float] = None) -> float:
        return float(a) + (b or 0.0)

    schema = json.loads(fn.spec.input_schema_json)
    assert schema["properties"]["a"] == {"type": "integer"}
    assert schema["properties"]["b"] == {"type": "number"}


def test_dispatch_with_optional_omitted():
    from ancora.tools import ToolRegistry

    @tool
    def greet(name: str, suffix: Optional[str] = None) -> str:
        return f"{name}{suffix or ''}"

    reg = ToolRegistry()
    reg.register(greet)
    result = reg.dispatch("greet", '{"name": "world"}')
    assert result == "world"
