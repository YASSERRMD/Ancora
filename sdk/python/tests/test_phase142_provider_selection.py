"""Phase 142 task 12: provider selection."""

import pytest
import ancora
from ancora.models import AgentSpec

ANTHROPIC_MODEL = "claude-opus-4-8"
OPENAI_MODEL = "gpt-4o"
GEMINI_MODEL = "gemini-2-5-pro"
MISTRAL_MODEL = "mistral-large-latest"
DEEPSEEK_MODEL = "deepseek-chat"


def test_agentspec_anthropic_model():
    spec = AgentSpec(name="anthro-agent", model_id=ANTHROPIC_MODEL)
    assert spec.model_id == ANTHROPIC_MODEL


def test_agentspec_openai_model():
    spec = AgentSpec(name="openai-agent", model_id=OPENAI_MODEL)
    assert spec.model_id == OPENAI_MODEL


def test_agentspec_gemini_model():
    spec = AgentSpec(name="gemini-agent", model_id=GEMINI_MODEL)
    assert spec.model_id == GEMINI_MODEL


def test_agentspec_mistral_model():
    spec = AgentSpec(name="mistral-agent", model_id=MISTRAL_MODEL)
    assert spec.model_id == MISTRAL_MODEL


def test_agentspec_deepseek_model():
    spec = AgentSpec(name="deepseek-agent", model_id=DEEPSEEK_MODEL)
    assert spec.model_id == DEEPSEEK_MODEL


def test_all_five_model_ids_are_distinct():
    models = [ANTHROPIC_MODEL, OPENAI_MODEL, GEMINI_MODEL, MISTRAL_MODEL, DEEPSEEK_MODEL]
    assert len(set(models)) == len(models)


@pytest.mark.asyncio
async def test_anthropic_agent_run_starts():
    rt = ancora.Runtime()
    spec = AgentSpec(name="anthro-run", model_id=ANTHROPIC_MODEL)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_openai_agent_run_starts():
    rt = ancora.Runtime()
    spec = AgentSpec(name="openai-run", model_id=OPENAI_MODEL)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_all_five_providers_start_runs():
    rt = ancora.Runtime()
    for model in [ANTHROPIC_MODEL, OPENAI_MODEL, GEMINI_MODEL, MISTRAL_MODEL, DEEPSEEK_MODEL]:
        spec = AgentSpec(name=f"prov-{model[:4]}", model_id=model)
        agent = ancora.Agent(rt, spec)
        run = await agent.run()
        assert run.run_id != "", f"Empty run ID for model {model}"
    rt.free()


def test_agentspec_builder_sets_model_id():
    spec = (
        ancora.AgentSpecBuilder()
        .with_name("builder-prov")
        .with_model_id(ANTHROPIC_MODEL)
        .build()
    )
    assert spec.model_id == ANTHROPIC_MODEL
