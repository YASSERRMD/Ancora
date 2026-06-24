"""Conformance runner example.

Runs all built-in conformance scenarios and prints a PASS/FAIL report.
Exits with a non-zero code if any scenario fails. Runs fully offline.

Usage::

    python -m examples.conformance_runner
"""

import asyncio
import sys

import ancora
from ancora import ConformanceSuite, register_builtin_scenarios


async def main() -> int:
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)

    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()

    print(suite.summary(results))

    return 0 if not suite.failed(results) else 1


if __name__ == "__main__":
    sys.exit(asyncio.run(main()))
