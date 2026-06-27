# Structured Output (Python)

Force the agent to return a Pydantic model instead of raw text.

## Define the output schema

```python
from pydantic import BaseModel

class AnalysisResult(BaseModel):
    headline: str
    sentiment: str
    confidence: float
```

## Pass the schema to AgentSpec

```python
from ancora import Runtime, AgentSpec

spec = AgentSpec(
    model="llama3",
    instructions="Analyse the sentiment of the user's message.",
    output_schema=AnalysisResult,
)

rt = Runtime()
result = rt.run(spec, "Ancora makes agent development simple!")
analysis = result.parse(AnalysisResult)
print(analysis.headline, analysis.sentiment, analysis.confidence)
```

## How it works

Ancora converts the Pydantic model to a JSON Schema and passes it to the
model as a response format constraint. The raw JSON output is automatically
validated and parsed.

## Nested models

```python
class Tag(BaseModel):
    name: str
    score: float

class Report(BaseModel):
    summary: str
    tags: list[Tag]

spec = AgentSpec(model="llama3", instructions="Tag this text.", output_schema=Report)
report = rt.run(spec, "...").parse(Report)
for tag in report.tags:
    print(tag.name, tag.score)
```

## See also

- [Tools](tools.md)
- [API reference](api-reference.md)
