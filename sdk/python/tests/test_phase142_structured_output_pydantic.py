"""Phase 142 task 6: structured output pydantic."""

import json
import pytest
from pydantic import BaseModel
from ancora.schema import params_to_schema
from ancora.models import AgentSpec, ToolSpec
from ancora.builder import AgentSpecBuilder, ToolSpecBuilder


class OutputSchema(BaseModel):
    name: str
    score: float
    tags: list[str]


class SimplePayload(BaseModel):
    value: str
    count: int


def test_pydantic_model_schema_is_valid_json():
    schema = OutputSchema.model_json_schema()
    assert "properties" in schema
    assert "name" in schema["properties"]


def test_pydantic_model_serialization_round_trip():
    obj = OutputSchema(name="test", score=0.95, tags=["a", "b"])
    raw = obj.model_dump_json()
    obj2 = OutputSchema.model_validate_json(raw)
    assert obj2.name == "test"
    assert obj2.score == 0.95
    assert obj2.tags == ["a", "b"]


def test_pydantic_schema_type_is_object():
    schema = OutputSchema.model_json_schema()
    assert schema.get("type") == "object"


def test_pydantic_schema_required_includes_all_fields():
    schema = OutputSchema.model_json_schema()
    required = set(schema.get("required", []))
    assert "name" in required
    assert "score" in required
    assert "tags" in required


def test_tool_spec_with_pydantic_input_schema():
    schema_str = json.dumps(SimplePayload.model_json_schema())
    ts = ToolSpec(name="structured-tool", description="test", input_schema_json=schema_str)
    assert ts.input_schema_json != ""


def test_agent_spec_with_output_schema():
    schema_str = json.dumps(OutputSchema.model_json_schema())
    spec = AgentSpec(name="so-agent", model_id="gpt-4o", output_schema_json=schema_str)
    assert spec.output_schema_json != ""


def test_agentspec_with_output_schema_direct():
    schema_str = json.dumps(OutputSchema.model_json_schema())
    spec = AgentSpec(
        name="out-agent",
        model_id="llama3",
        output_schema_json=schema_str,
    )
    assert spec.output_schema_json != ""


def test_simple_payload_parse():
    obj = SimplePayload(value="hello", count=3)
    assert obj.value == "hello"
    assert obj.count == 3


def test_pydantic_model_validates_required_fields():
    from pydantic import ValidationError
    with pytest.raises(ValidationError):
        OutputSchema.model_validate({"score": 1.0})


def test_params_to_schema_returns_schema_string():
    schema = params_to_schema({"x": str, "y": int})
    assert isinstance(schema, str)
    parsed = json.loads(schema)
    assert "properties" in parsed
