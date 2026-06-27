"""Phase 142 task 7: multi-agent verifier."""

import pytest
import ancora
from ancora.models import AgentSpec, ToolSpec
from ancora.conformance import ConformanceSuite


def test_verifier_toolspec_can_be_built():
    ts = ToolSpec(name="verify", description="verifies a claim")
    assert ts.name == "verify"


def test_agent_with_verifier_tool():
    spec = AgentSpec(
        name="verifier-agent",
        model_id="llama3",
        tools=[ToolSpec(name="verify", description="verifies a claim")],
    )
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "verify"


def test_two_tool_specs_can_be_added():
    spec = AgentSpec(
        name="multi-tool",
        model_id="llama3",
        tools=[
            ToolSpec(name="search", description="searches"),
            ToolSpec(name="verify", description="verifies"),
        ],
    )
    assert len(spec.tools) == 2


def test_conformance_suite_can_be_created():
    suite = ConformanceSuite()
    assert suite is not None


def test_conformance_suite_names_is_list():
    suite = ConformanceSuite()
    assert isinstance(suite.names, list)


@pytest.mark.asyncio
async def test_conformance_suite_run_all_returns_dict():
    rt = ancora.Runtime()
    suite = ConformanceSuite()
    results = await suite.run_all(rt)
    assert isinstance(results, dict)
    rt.free()


@pytest.mark.asyncio
async def test_verifier_agent_run_starts():
    rt = ancora.Runtime()
    spec = AgentSpec(
        name="verifier",
        model_id="llama3",
        tools=[ToolSpec(name="verify", description="verifier tool")],
    )
    agent = ancora.Agent(rt, spec)
    run = await agent.run()
    assert run.run_id != ""
    rt.free()


def test_verifier_tool_spec_in_agent_spec():
    from ancora.builder import AgentSpecBuilder, ToolSpecBuilder
    ts = ToolSpecBuilder().with_name("verify").with_description("verifier").build()
    spec = AgentSpecBuilder().with_name("v-agent").with_model_id("llama3").with_tool(ts).build()
    assert len(spec.tools) == 1
    assert spec.tools[0].name == "verify"


def test_conformance_suite_register_scenario():
    suite = ConformanceSuite()

    async def my_scenario(rt):
        return True

    suite.register("my-scenario", my_scenario)
    assert "my-scenario" in suite.names


@pytest.mark.asyncio
async def test_conformance_suite_run_registered_scenario():
    rt = ancora.Runtime()
    suite = ConformanceSuite()

    async def always_pass(r):
        return True

    suite.register("always-pass", always_pass)
    results = await suite.run_all(rt)
    assert results["always-pass"] is True
    rt.free()


def test_conformance_suite_summary_is_string():
    suite = ConformanceSuite()
    summary = suite.summary({"test": True, "other": False})
    assert isinstance(summary, str)
    assert "1/2" in summary
