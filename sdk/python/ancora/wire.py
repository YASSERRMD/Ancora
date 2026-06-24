"""Serialize and deserialize Ancora spec models to and from wire bytes (JSON)."""

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
