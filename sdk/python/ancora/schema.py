"""Convert Python type hints to JSON Schema objects.

Supports a subset of Python types sufficient for Ancora tool parameters:
``str``, ``int``, ``float``, ``bool``, ``list``, ``dict``, and
``Optional[T]``.  Unknown annotations are mapped to an empty schema ``{}``.
"""

from __future__ import annotations

import inspect
import typing
from typing import Any, Dict, get_args, get_origin


def type_to_json_schema(tp: Any) -> Dict[str, Any]:
    """Convert a Python type annotation to a JSON Schema dict."""
    if tp is inspect.Parameter.empty or tp is type(None):
        return {}
    origin = get_origin(tp)
    if origin is typing.Union:
        args = [a for a in get_args(tp) if a is not type(None)]
        if len(args) == 1:
            return type_to_json_schema(args[0])
        return {"anyOf": [type_to_json_schema(a) for a in args]}
    if tp is str:
        return {"type": "string"}
    if tp is int:
        return {"type": "integer"}
    if tp is float:
        return {"type": "number"}
    if tp is bool:
        return {"type": "boolean"}
    if tp is list or origin is list:
        return {"type": "array"}
    if tp is dict or origin is dict:
        return {"type": "object"}
    return {}


def params_to_schema(sig: inspect.Signature) -> Dict[str, Any]:
    """Build a JSON Schema for the parameters of a function signature."""
    properties: Dict[str, Any] = {}
    required = []
    for name, param in sig.parameters.items():
        if name == "self":
            continue
        schema = type_to_json_schema(param.annotation)
        properties[name] = schema
        origin = get_origin(param.annotation)
        has_default = param.default is not inspect.Parameter.empty
        is_optional = origin is typing.Union and type(None) in get_args(param.annotation)
        if not has_default and not is_optional:
            required.append(name)
    result: Dict[str, Any] = {"type": "object", "properties": properties}
    if required:
        result["required"] = required
    return result
