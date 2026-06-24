"""Ancora Python SDK.

Provides a :class:`Runtime` handle and :func:`version` for the Ancora
agent runtime, backed by a native Rust extension via PyO3.

Example::

    import ancora
    with ancora.Runtime() as rt:
        print(rt)
"""

from ancora._ancora import AncorError, Runtime, version
from ancora.agent import Agent
from ancora.builder import AgentSpecBuilder, ToolSpecBuilder
from ancora.memory import MemoryStore
from ancora.models import AgentSpec, EffectClass, RetryPolicy, StreamEvent, ToolSpec
from ancora.run import Run
from ancora.tools import Tool, ToolRegistry, tool
from ancora.wire import from_wire_bytes, to_wire_bytes

__all__ = [
    "AncorError",
    "Agent",
    "AgentSpec",
    "AgentSpecBuilder",
    "EffectClass",
    "MemoryStore",
    "RetryPolicy",
    "Run",
    "Runtime",
    "StreamEvent",
    "Tool",
    "ToolRegistry",
    "ToolSpec",
    "ToolSpecBuilder",
    "from_wire_bytes",
    "to_wire_bytes",
    "tool",
    "version",
]
