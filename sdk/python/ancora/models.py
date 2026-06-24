"""Pydantic models mirroring the Ancora agent spec protobuf contracts."""

from __future__ import annotations

import enum
from typing import List, Optional

from pydantic import BaseModel, Field


class EffectClass(enum.IntEnum):
    """Classifies the observable side effect a tool may produce."""

    UNSPECIFIED = 0
    PURE = 1
    READ = 2
    WRITE = 3


class ToolSpec(BaseModel):
    """Describes a single tool that an agent may invoke."""

    name: str = ""
    description: str = ""
    input_schema_json: str = ""
    output_schema_json: str = ""
    effect_class: EffectClass = EffectClass.UNSPECIFIED
    idempotency_key_template: str = ""
