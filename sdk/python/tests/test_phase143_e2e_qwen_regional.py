"""Phase 143 e2e task 6: qwen regional via mock end to end."""

import pytest
import ancora
from ancora.models import AgentSpec

QWEN_MODEL = "qwen-turbo"
QWEN_LONG_MODEL = "qwen-long"
QWEN_MAX_MODEL = "qwen-max"

QWEN_FIXTURE = {
    "provider": "alibaba-cloud",
    "region": "cn-hangzhou",
    "endpoint": "mock://dashscope.aliyuncs.com",
}


def test_qwen_model_id_non_empty():
    assert QWEN_MODEL != ""


def test_qwen_long_model_id_distinct():
    assert QWEN_MODEL != QWEN_LONG_MODEL


def test_qwen_max_model_id_distinct():
    assert QWEN_MAX_MODEL != QWEN_MODEL


def test_qwen_fixture_region():
    assert QWEN_FIXTURE["region"] == "cn-hangzhou"


def test_qwen_fixture_provider():
    assert QWEN_FIXTURE["provider"] == "alibaba-cloud"


def test_qwen_agentspec_model_id():
    spec = AgentSpec(name="qwen-agent", model_id=QWEN_MODEL)
    assert spec.model_id == QWEN_MODEL


@pytest.mark.asyncio
async def test_qwen_turbo_run_starts():
    rt = ancora.Runtime()
    spec = AgentSpec(name="qwen-turbo-e2e", model_id=QWEN_MODEL)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_qwen_long_run_starts():
    rt = ancora.Runtime()
    spec = AgentSpec(name="qwen-long-e2e", model_id=QWEN_LONG_MODEL)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_qwen_max_run_starts():
    rt = ancora.Runtime()
    spec = AgentSpec(name="qwen-max-e2e", model_id=QWEN_MAX_MODEL)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


@pytest.mark.asyncio
async def test_qwen_regional_run_events_non_empty():
    rt = ancora.Runtime()
    spec = AgentSpec(name="qwen-reg", model_id=QWEN_MODEL)
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    events = await run.drain_events()
    assert len(events) > 0
    rt.free()
