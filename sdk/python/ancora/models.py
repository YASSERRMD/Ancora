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
