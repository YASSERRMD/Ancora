# Spec Round-Trip Notes

This document describes how a graph spec survives the serialize / parse cycle.

## Text Format

`spec_to_text` produces a line-based format:

```
name <graph_name>
node <id> <kind> <label> <x> <y>
edge <id> <source> <target> <edge_type>
```

Rules:
- `<label>` must not contain spaces; spaces are replaced with underscores on export.
- `<x>` and `<y>` are floating-point values (integer-looking floats are fine).
- Lines starting with `#` and blank lines are ignored by the parser.

## Round-Trip Guarantees

The following properties are preserved:

1. **Node count** - all nodes survive the cycle.
2. **Node kinds** - the `kind` field is never modified.
3. **Edge count** - all edges survive.
4. **Edge types** - `EdgeType::from_str(t.to_string())` is the identity for all variants.

The following properties are NOT guaranteed by the text format:

- **Node label fidelity** - spaces in labels are replaced by underscores.
- **Position precision** - floating-point formatting may lose sub-pixel precision.
- **Node config** - per-node key-value config is not serialized by the text format.

## JSON Upgrade Path

A future `spec_to_json` / `parse_json_spec` pair can preserve all
properties including config, labels with spaces, and fractional positions.
This is straightforward to add without breaking the existing text format.

## Validation After Import

Always call `validate_spec` after `parse_simple_spec` or `parse_json_spec`.
The parser is permissive; the validator enforces semantic rules:

- No empty spec name.
- No duplicate node IDs.
- All edge sources and targets must reference existing nodes.
- No self-loops.

Isolated nodes (nodes with no edges, when the graph has more than one node)
produce a warning, not an error.
