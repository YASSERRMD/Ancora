# Graph Builder Guide

The `ancora-builder` crate provides a visual graph builder for the Ancora
agent framework. It lets you compose agent pipelines graphically and run
them locally against the offline stub backend.

## Core Concepts

### Canvas and Viewport

The builder renders nodes on an infinite 2-D canvas. The `Viewport` tracks
pan offset and zoom level. All canvas positions are in logical pixels;
the viewport converts them to screen pixels for rendering.

### Palette

The `Palette` contains all registered node kinds:

- **Agents** (`agent.*`): LLM, retrieval, classifier.
- **Tools** (`tool.*`): web search, code executor, file reader.
- **Verifiers** (`verifier.*`): JSON schema, hallucination detector, toxicity filter.
- **Control** (`control.*`): router, merge, loop.

### Node Placement

Drag a node from the palette onto the canvas. The `Canvas` assigns a
unique ID and records the position. You can snap to a configurable grid
with `Canvas::snap_to_grid`.

### Edges

Edges carry a type: `DataFlow`, `ControlDep`, `Verification`, or
`LoopBack`. The `EdgeStore` enforces connection rules: you can register
which source kinds are allowed to connect to which target kinds.
Self-loops and duplicate edges are always rejected.

### Configuration Panels

Each node kind has an associated `ConfigPanel` with typed fields (text,
number, bool, select, secret). The `PanelEditSession` validates required
fields before the spec is saved.

## Import and Export

Use `export_spec` to serialize the canvas to a `GraphSpec`. Use
`import_spec` to reconstruct a canvas from a saved spec. The simple
line-based text format supports round-tripping via `spec_to_text` /
`parse_simple_spec`.

## Running a Graph

Call `run_spec` with a `LocalBackendConfig` to execute the graph. In
offline mode (default) each node is simulated: no network calls are made.
The result includes per-step outputs and a `TraceOverlay` you can display
on the canvas.

## Validation

Always validate a spec with `validate_spec` before saving or running.
The `ValidationReport` collects errors and warnings with optional node/
edge targets for inline highlighting in the editor.

## Templates

`TemplateRegistry::default_registry()` ships several ready-made graphs:

| ID | Description |
|----|-------------|
| `single_agent` | Minimal LLM agent |
| `rag_pipeline` | Retrieval-augmented generation |
| `agent_verifier` | Agent with schema and toxicity verifiers |
| `multi_agent` | Fan-out to multiple agents |
| `loop_template` | Self-refining agent loop |

Call `registry.instantiate(id, new_name)` to clone a template's spec and
customise it.
