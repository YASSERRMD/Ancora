"""Pydantic models mirroring the Ancora agent spec protobuf contracts.

The field names and semantics match the protobuf definitions in
``crates/ancora-proto/proto/contracts.proto``. All models use snake_case
fields matching the JSON wire format expected by the Ancora runtime.

Typical usage::

    from ancora.models import AgentSpec, ToolSpec, EffectClass

    spec = AgentSpec(
        name="my-agent",
        model_id="llama3",
        tools=[ToolSpec(name="search", effect_class=EffectClass.READ)],
    )
"""

from __future__ import annotations

import enum
from typing import List, Optional

from pydantic import BaseModel, ConfigDict, Field


class EffectClass(enum.IntEnum):
    """Classifies the observable side effect a tool may produce."""

    UNSPECIFIED = 0
    PURE = 1
    READ = 2
    WRITE = 3


class ToolSpec(BaseModel):
    """Describes a single tool that an agent may invoke."""

    model_config = ConfigDict(extra="forbid")

    name: str = ""
    description: str = ""
    input_schema_json: str = ""
    output_schema_json: str = ""
    effect_class: EffectClass = EffectClass.UNSPECIFIED
    idempotency_key_template: str = ""


class RetryPolicy(BaseModel):
    """Retry policy attached to a model call or tool call."""

    model_config = ConfigDict(extra="forbid")

    max_attempts: int = Field(default=0, ge=0)
    initial_backoff_ms: int = Field(default=0, ge=0)
    max_backoff_ms: int = Field(default=0, ge=0)
    jitter: float = Field(default=0.0, ge=0.0, le=1.0)


class StreamEvent(BaseModel):
    """A single event emitted by a streaming run.

    The ``kind`` field identifies the event type:

    - ``"started"`` -- run has begun; ``spec`` contains the serialized agent spec.
    - ``"token"`` -- a streamed token; ``text`` contains the token string.
    - ``"completed"`` -- run finished successfully.
    - ``"resumed"`` -- run was resumed after a human-in-the-loop pause.
    """

    model_config = ConfigDict(extra="allow")

    kind: str = ""
    run_id: str = ""
    text: Optional[str] = None
    spec: Optional[str] = None
    decision: Optional[str] = None

    @classmethod
    def from_bytes(cls, data: bytes) -> "StreamEvent":
        """Parse a raw event bytes value into a StreamEvent."""
        return cls.model_validate_json(data)

    @property
    def is_token(self) -> bool:
        """Return True if this is a token event."""
        return self.kind == "token"

    @property
    def is_started(self) -> bool:
        """Return True if this is a started event."""
        return self.kind == "started"

    @property
    def is_completed(self) -> bool:
        """Return True if this is a completed event."""
        return self.kind == "completed"


class AgentSpec(BaseModel):
    """Specifies a single agent in the graph."""

    model_config = ConfigDict(extra="forbid")

    name: str = ""

    def wire_bytes(self) -> bytes:
        """Return this spec serialized to JSON wire bytes."""
        from ancora.wire import to_wire_bytes
        return to_wire_bytes(self)

    model_id: str = ""
    instructions: str = ""
    output_schema_json: str = ""
    tools: List[ToolSpec] = Field(default_factory=list)
    max_steps: int = Field(default=0, ge=0)
    model_retry: Optional[RetryPolicy] = None
    model_params_json: str = ""
