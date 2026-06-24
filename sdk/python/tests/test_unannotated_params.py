"""Tests for tools with unannotated or partially annotated parameters."""

import json

from ancora.schema import params_to_schema
from ancora.tools import tool


def test_no_params_produces_empty_properties():
    @tool
    def ping() -> str:
        return "pong"

    schema = json.loads(ping.spec.input_schema_json)
    assert schema["type"] == "object"
    assert schema["properties"] == {}
    assert "required" not in schema


def test_unannotated_param_maps_to_empty_schema():
    @tool
    def greet(name) -> str:  # no annotation on name
        return f"hello {name}"

    schema = json.loads(greet.spec.input_schema_json)
    assert schema["properties"]["name"] == {}


def test_mixed_annotated_and_unannotated():
    @tool
    def mix(x: int, y) -> int:
        return x

    schema = json.loads(mix.spec.input_schema_json)
    assert schema["properties"]["x"] == {"type": "integer"}
    assert schema["properties"]["y"] == {}
    assert schema.get("required") == ["x", "y"]


def test_tool_with_no_params_is_callable():
    @tool
    def noop():
        return 42

    assert noop() == 42
    assert noop.call_with_json("{}") == 42
    assert noop.call_with_kwargs() == 42
