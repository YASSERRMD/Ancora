#!/usr/bin/env python3
"""
Cross-language journal comparison tool.

Usage:
    compare.py journal-a.jsonl journal-b.jsonl

Each file is a JSON Lines file where each line is a journal event with at
least a "kind" field (and optionally a "node_id" field).  Model-generated
content fields (text, result, output) are ignored.

Exit 0 when the two journals are structurally identical.
Exit 1 and print a description on mismatch.
"""

import json
import sys


MASKED_FIELDS = {"text", "result", "output", "output_json", "result_json",
                 "message", "detail", "input_json"}


def load_events(path: str) -> list[dict]:
    events = []
    with open(path) as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            obj = json.loads(line)
            kind = obj.get("kind") or obj.get("event", {}).get("kind", "unknown")
            node_id = (
                obj.get("node_id")
                or obj.get("event", {}).get("node_id")
                or obj.get("activity_key")
                or obj.get("event", {}).get("activity_key")
            )
            events.append({"kind": kind, "node_id": node_id})
    return events


def compare(a: list[dict], b: list[dict]) -> str | None:
    if len(a) != len(b):
        return f"event count mismatch: left={len(a)} right={len(b)}"
    for i, (ea, eb) in enumerate(zip(a, b)):
        if ea["kind"] != eb["kind"]:
            return (f"kind mismatch at index {i}: "
                    f"left={ea['kind']!r} right={eb['kind']!r}")
        if ea["node_id"] != eb["node_id"]:
            return (f"node_id mismatch at index {i}: "
                    f"left={ea['node_id']!r} right={eb['node_id']!r}")
    return None


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} journal-a.jsonl journal-b.jsonl", file=sys.stderr)
        return 2
    a_path, b_path = sys.argv[1], sys.argv[2]
    a = load_events(a_path)
    b = load_events(b_path)
    err = compare(a, b)
    if err:
        print(f"MISMATCH: {err}", file=sys.stderr)
        return 1
    print(f"OK: {len(a)} events match structurally")
    return 0


if __name__ == "__main__":
    sys.exit(main())
