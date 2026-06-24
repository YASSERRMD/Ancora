"""Shared helpers for Ancora Python SDK examples."""

from __future__ import annotations

import json
from typing import Any


def print_event(raw: bytes) -> None:
    """Print a single raw event bytes to stdout."""
    ev = json.loads(raw)
    kind = ev.get("kind", "?")
    if kind == "token":
        print(f"  token: {ev.get('text', '')!r}")
    elif kind == "started":
        print(f"  started  run_id={ev.get('run_id', '')[:8]}...")
    elif kind == "completed":
        print("  completed")
    elif kind == "resumed":
        print(f"  resumed  decision={ev.get('decision', '')!r}")
    else:
        print(f"  {kind}")


def pretty_results(results: dict[str, bool]) -> str:
    """Format a run_all results dict as a table."""
    lines = []
    for name, passed in results.items():
        mark = "PASS" if passed else "FAIL"
        lines.append(f"  {mark}  {name}")
    total = len(results)
    n_pass = sum(1 for ok in results.values() if ok)
    lines.append(f"{n_pass}/{total} passed")
    return "\n".join(lines)
