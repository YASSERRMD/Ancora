"""Structured typed output with Pydantic example.

Demonstrates building a JSON Schema from a Pydantic model and embedding it
in the agent system prompt so the agent knows the expected output shape.
Runs fully offline.

Usage::

    python -m examples.structured_output
"""

from __future__ import annotations

import asyncio
import json
from typing import List, Optional

import ancora
from pydantic import BaseModel, Field


class AnalysisResult(BaseModel):
    """Expected agent output shape for an analysis task."""

    summary: str = Field(description="One-sentence summary of the analysis")
    topics: List[str] = Field(description="List of main topics identified")
    confidence: float = Field(description="Confidence score between 0.0 and 1.0")
    action_item: str = Field(description="Recommended next action")


class ClassificationResult(BaseModel):
    """Expected agent output shape for a classification task."""

    label: str = Field(description="Primary classification label")
    subcategory: Optional[str] = Field(default=None, description="More specific subcategory")
    score: int = Field(description="Integer confidence 0-100")


def model_to_json_schema(model: type[BaseModel]) -> str:
    """Return a JSON Schema string for a Pydantic model."""
    schema = model.model_json_schema()
    return json.dumps(schema, indent=2)


async def run_with_schema(rt: ancora.Runtime, schema_json: str, name: str) -> None:
    system = (
        f"You are an analysis agent. Always respond with valid JSON matching "
        f"this schema:\n{schema_json}"
    )
    spec = ancora.AgentSpec(
        name=name,
        model_id="local-model",
        instructions=system,
    )
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    print(f"started run: {run.run_id}")
    events = await run.drain_events()
    print(f"received {len(events)} event(s)")


async def main() -> None:
    analysis_schema = model_to_json_schema(AnalysisResult)
    classification_schema = model_to_json_schema(ClassificationResult)
    print(f"analysis schema fields: {list(AnalysisResult.model_fields)}")
    print(f"classification schema fields: {list(ClassificationResult.model_fields)}")

    rt = ancora.Runtime()
    await run_with_schema(rt, analysis_schema, "analysis-agent")
    await run_with_schema(rt, classification_schema, "classification-agent")
    rt.free()
    print("done.")


if __name__ == "__main__":
    asyncio.run(main())
