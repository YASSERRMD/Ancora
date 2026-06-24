"""Conformance test suite for the Ancora Python SDK.

A :class:`ConformanceSuite` collects named scenarios and runs them against a
live :class:`~ancora._ancora.Runtime`. Each scenario returns ``True`` when it
passes and ``False`` (or raises) when it fails.

Typical usage::

    import asyncio
    from ancora import Runtime
    from ancora.conformance import ConformanceSuite

    suite = ConformanceSuite()

    async def main():
        rt = Runtime()
        results = await suite.run_all(rt)
        for name, passed in results.items():
            print(name, "PASS" if passed else "FAIL")
        rt.free()

    asyncio.run(main())
"""

from __future__ import annotations

import asyncio
import json
from typing import Awaitable, Callable, Dict, Optional

import ancora
from ancora.memory import MemoryStore
from ancora.models import AgentSpec, StreamEvent


ScenarioFn = Callable[["ancora._ancora.Runtime"], Awaitable[bool]]


class ConformanceSuite:
    """Registry of named async conformance scenarios.

    Scenarios are registered via :meth:`register` or the :func:`scenario`
    decorator.  Call :meth:`run_all` to execute every registered scenario
    against a runtime and collect results.
    """

    def __init__(self) -> None:
        self._scenarios: Dict[str, ScenarioFn] = {}

    def register(self, name: str, fn: ScenarioFn) -> None:
        """Register a scenario function under *name*."""
        self._scenarios[name] = fn

    @property
    def names(self) -> list[str]:
        """Return scenario names in registration order."""
        return list(self._scenarios.keys())

    async def run_scenario(self, name: str, rt: "ancora._ancora.Runtime") -> bool:
        """Run a single scenario by name. Returns True on pass."""
        fn = self._scenarios.get(name)
        if fn is None:
            raise KeyError(f"No scenario registered: {name!r}")
        try:
            return bool(await fn(rt))
        except Exception:
            return False

    async def run_all(
        self, rt: "ancora._ancora.Runtime"
    ) -> Dict[str, bool]:
        """Run all registered scenarios and return a name -> pass dict."""
        results: Dict[str, bool] = {}
        for name in self._scenarios:
            results[name] = await self.run_scenario(name, rt)
        return results
