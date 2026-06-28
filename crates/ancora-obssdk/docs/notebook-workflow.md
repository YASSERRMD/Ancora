# Notebook Workflow for Observability

ancora-obssdk provides a notebook render path for inline trace visualization in Jupyter and similar environments.

## Overview

The `NotebookTraceRenderer` converts a `Trace` into cell output in three formats:
- Plain text (ASCII table)
- HTML table (rich display)
- Markdown table

## Usage in a Notebook

```python
# Python notebook cell
from ancora_obssdk.notebook import NotebookTraceRenderer

renderer = NotebookTraceRenderer()

# Plain text output
output = renderer.render_plain(trace)
print(output.content)

# HTML output (rendered inline in Jupyter)
html_output = renderer.render_html(trace)
from IPython.display import HTML, display
display(HTML(html_output.content))

# Markdown
md_output = renderer.render_markdown(trace)
from IPython.display import Markdown, display
display(Markdown(md_output.content))
```

## Output Formats

### Plain Text

```
Trace: my-trace-id
Spans: 2
---
  [(root)] root.op | 2000ns
  [s1] sub.op | 1600ns
```

### HTML

Renders as a sortable table with columns: ID, Name, Parent, Duration.

### Markdown

```markdown
## Trace `my-trace-id`

| ID | Name | Parent | Duration |
|----|------|--------|----------|
| s1 | root.op | (root) | 2000ns |
| s2 | sub.op | s1 | 1600ns |
```

## Integration with Eval

You can chain notebook rendering with eval results:

```python
result = eval_runner.evaluate(trace, criteria)
if not result.passed:
    display(HTML(renderer.render_html(trace).content))
    print(f"Eval failed: {result.notes}")
```
