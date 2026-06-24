"""Fluent builder for AgentSpec and ToolSpec."""

from __future__ import annotations

from typing import List

from ancora.models import AgentSpec, EffectClass, RetryPolicy, ToolSpec


class ToolSpecBuilder:
    """Fluent builder for ToolSpec."""

    def __init__(self) -> None:
        self._name: str = ""
        self._description: str = ""
        self._input_schema_json: str = ""
        self._output_schema_json: str = ""
        self._effect_class: EffectClass = EffectClass.UNSPECIFIED
        self._idempotency_key_template: str = ""

    def with_name(self, name: str) -> "ToolSpecBuilder":
        self._name = name
        return self

    def with_description(self, description: str) -> "ToolSpecBuilder":
        self._description = description
        return self

    def with_input_schema(self, schema: str) -> "ToolSpecBuilder":
        self._input_schema_json = schema
        return self

    def with_output_schema(self, schema: str) -> "ToolSpecBuilder":
        self._output_schema_json = schema
        return self

    def with_effect_class(self, effect_class: EffectClass) -> "ToolSpecBuilder":
        self._effect_class = effect_class
        return self

    def build(self) -> ToolSpec:
        return ToolSpec(
            name=self._name,
            description=self._description,
            input_schema_json=self._input_schema_json,
            output_schema_json=self._output_schema_json,
            effect_class=self._effect_class,
            idempotency_key_template=self._idempotency_key_template,
        )


class AgentSpecBuilder:
    """Fluent builder for AgentSpec."""

    def __init__(self) -> None:
        self._name: str = ""
        self._model_id: str = ""
        self._instructions: str = ""
        self._output_schema_json: str = ""
        self._tools: List[ToolSpec] = []
        self._max_steps: int = 0
        self._model_retry: RetryPolicy | None = None
        self._model_params_json: str = ""

    def with_name(self, name: str) -> "AgentSpecBuilder":
        self._name = name
        return self

    def with_model_id(self, model_id: str) -> "AgentSpecBuilder":
        self._model_id = model_id
        return self

    def with_instructions(self, instructions: str) -> "AgentSpecBuilder":
        self._instructions = instructions
        return self

    def with_tool(self, tool: ToolSpec) -> "AgentSpecBuilder":
        self._tools.append(tool)
        return self

    def with_max_steps(self, max_steps: int) -> "AgentSpecBuilder":
        self._max_steps = max_steps
        return self

    def with_model_retry(self, retry: RetryPolicy) -> "AgentSpecBuilder":
        self._model_retry = retry
        return self

    def build(self) -> AgentSpec:
        return AgentSpec(
            name=self._name,
            model_id=self._model_id,
            instructions=self._instructions,
            output_schema_json=self._output_schema_json,
            tools=list(self._tools),
            max_steps=self._max_steps,
            model_retry=self._model_retry,
            model_params_json=self._model_params_json,
        )
