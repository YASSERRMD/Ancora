"""Async run handle returned by Agent.run()."""

from __future__ import annotations

import asyncio
from typing import TYPE_CHECKING, Optional

if TYPE_CHECKING:
    from ancora._ancora import Runtime


class Run:
    """Async handle to a single agent run.

    Iterate over events with ``async for event in run`` or collect all
    events at once with ``await run.drain_events()``.
    """

    def __init__(self, rt: "Runtime", run_id: str) -> None:
        self._rt = rt
        self._run_id = run_id

    @property
    def run_id(self) -> str:
        """Return the stable run identifier."""
        return self._run_id

    def __repr__(self) -> str:
        return f"Run(id={self._run_id!r})"

    def __aiter__(self) -> "Run":
        return self

    async def __anext__(self) -> bytes:
        await asyncio.sleep(0)
        ev = self._rt.poll_run(self._run_id)
        if ev is None:
            raise StopAsyncIteration
        return bytes(ev)

    async def resume(self, decision: bytes = b"") -> None:
        """Post a human decision to a suspended run."""
        await asyncio.sleep(0)
        self._rt.resume_run(self._run_id, decision)

    async def approve(self, comment: str = "") -> None:
        """Approve a suspended run, allowing it to continue."""
        import json
        payload = {"decision": "approved"}
        if comment:
            payload["comment"] = comment
        await self.resume(json.dumps(payload).encode())

    async def reject(self, reason: str = "") -> None:
        """Reject a suspended run, halting it."""
        import json
        payload = {"decision": "rejected"}
        if reason:
            payload["reason"] = reason
        await self.resume(json.dumps(payload).encode())

    async def drain_events(self) -> list[bytes]:
        """Collect all remaining events and return them as a list."""
        events: list[bytes] = []
        async for ev in self:
            events.append(ev)
        return events

    async def stream_events(self):
        """Async generator that yields each raw event bytes as it arrives."""
        async for ev in self:
            yield ev

    async def stream_tokens(self):
        """Async generator that yields token text strings from token events."""
        import json
        async for ev in self:
            try:
                obj = json.loads(ev)
            except (ValueError, KeyError):
                continue
            if obj.get("kind") == "token":
                text = obj.get("text")
                if text is not None:
                    yield text
