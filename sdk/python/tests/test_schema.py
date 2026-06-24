"""Tests for type-hint to JSON schema conversion."""

from typing import List, Optional
from ancora.schema import type_to_json_schema, params_to_schema
import inspect


def test_str_type():
    assert type_to_json_schema(str) == {"type": "string"}


def test_int_type():
    assert type_to_json_schema(int) == {"type": "integer"}


def test_float_type():
    assert type_to_json_schema(float) == {"type": "number"}


def test_bool_type():
    assert type_to_json_schema(bool) == {"type": "boolean"}


def test_list_type():
    assert type_to_json_schema(list) == {"type": "array"}


def test_dict_type():
    assert type_to_json_schema(dict) == {"type": "object"}


def test_optional_str():
    schema = type_to_json_schema(Optional[str])
    assert schema == {"type": "string"}


def test_params_to_schema_simple():
    def fn(query: str, limit: int): ...
    sig = inspect.signature(fn)
    schema = params_to_schema(sig)
    assert schema["type"] == "object"
    assert schema["properties"]["query"] == {"type": "string"}
    assert schema["properties"]["limit"] == {"type": "integer"}
    assert "query" in schema["required"]
    assert "limit" in schema["required"]


def test_params_to_schema_optional_not_required():
    def fn(query: str, limit: Optional[int] = None): ...
    sig = inspect.signature(fn)
    schema = params_to_schema(sig)
    assert "query" in schema["required"]
    assert "limit" not in schema.get("required", [])


def test_params_to_schema_with_default_not_required():
    def fn(query: str, limit: int = 10): ...
    sig = inspect.signature(fn)
    schema = params_to_schema(sig)
    assert "limit" not in schema.get("required", [])


def test_params_to_schema_no_params():
    def fn(): ...
    sig = inspect.signature(fn)
    schema = params_to_schema(sig)
    assert schema["properties"] == {}
