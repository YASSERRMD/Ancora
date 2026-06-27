"""Cost and OpenTelemetry trace example.

Demonstrates wrapping an agent run in lightweight span tracking to record
event counts, token estimates, and duration -- mirroring what an OTEL
exporter would collect. No OTEL SDK dependency required.
Runs fully offline.

Usage::

    python -m examples.cost_otel
"""

from __future__ import annotations

import asyncio
import time
from typing import Any


import ancora


class Span:
    """Minimal stand-in for an OTEL span."""

    def __init__(self, name: str) -> None:
        self.name = name
        self._started = time.monotonic()
        self._attrs: dict[str, Any] = {}

    def set_attribute(self, key: str, value: Any) -> None:
        self._attrs[key] = value

    def end(self) -> None:
        dur_ms = (time.monotonic() - self._started) * 1000
        parts = [f"span={self.name!r}", f"duration_ms={dur_ms:.1f}"]
        for k, v in self._attrs.items():
            parts.append(f"{k}={v!r}")
        print("  " + "  ".join(parts))


def estimate_tokens(data: bytes) -> int:
    """Estimate token count from raw bytes using a 4-bytes-per-token heuristic."""
    return max(1, len(data) // 4)


async def main() -> None:
    rt = ancora.Runtime()
    spec = ancora.AgentSpec(
        name="cost-agent",
        model_id="local-model",
        instructions="Respond concisely.",
    )
    agent = ancora.Agent(rt, spec)

    root = Span("agent.run")

    drain_span = Span("agent.drain_events")
    run = await agent.run()
    root.set_attribute("run.id", run.run_id)
    print(f"run started: {run.run_id}")

    events = await run.drain_events()
    drain_span.set_attribute("event.count", len(events))
    drain_span.end()

    total_bytes = sum(len(e) for e in events)
    total_tokens = sum(estimate_tokens(e) for e in events)
    root.set_attribute("event.count", len(events))
    root.set_attribute("bytes.total", total_bytes)
    root.set_attribute("tokens.estimated", total_tokens)
    root.end()

    summary = Span("agent.summary")
    summary.set_attribute("events", len(events))
    summary.set_attribute("tokens.estimated", total_tokens)
    summary.end()

    print(f"events: {len(events)}  estimated_tokens: {total_tokens}")
    rt.free()
    print("cost-otel done.")


if __name__ == "__main__":
    asyncio.run(main())
