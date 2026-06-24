"""Serialize and deserialize Ancora spec models to and from wire bytes (JSON).

The wire format is UTF-8 encoded JSON with snake_case field names.  This matches
the JSON representation that the Ancora runtime uses when unmarshaling agent specs
on the Rust side via ``serde_json``.

Example::

    from ancora.wire import to_wire_bytes, from_wire_bytes
    from ancora.models import AgentSpec

    spec = AgentSpec(name="a", model_id="m")
    wire = to_wire_bytes(spec)    # b'{"name":"a","model_id":"m",...}'
    back = from_wire_bytes(wire)  # AgentSpec(name='a', model_id='m', ...)
"""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from ancora.models import AgentSpec


def to_wire_bytes(spec: "AgentSpec") -> bytes:
    """Serialize an AgentSpec to JSON wire bytes.

    The format matches the JSON representation expected by the Ancora runtime:
    snake_case field names, effect_class as integer, omit None values.
    """
    return spec.model_dump_json(exclude_none=True).encode("utf-8")


def from_wire_bytes(data: bytes) -> "AgentSpec":
    """Deserialize an AgentSpec from JSON wire bytes."""
    from ancora.models import AgentSpec
    return AgentSpec.model_validate_json(data)
