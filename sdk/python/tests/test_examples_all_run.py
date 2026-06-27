"""Smoke test: every example in the examples package runs without error."""

import pytest

from examples.single_agent import main as single_agent
from examples.rag_memory import main as rag_memory
from examples.mcp_tool_use import main as mcp_tool_use
from examples.streaming import main as streaming
from examples.human_in_loop import main as human_in_loop
from examples.multi_agent import main as multi_agent
from examples.conformance_runner import main as conformance_runner
from examples.tool_composition import main as tool_composition
from examples.async_tools import main as async_tools
from examples.structured_output import main as structured_output
from examples.qwen_gateway import main as qwen_gateway
from examples.durable_restart import main as durable_restart
from examples.cost_otel import main as cost_otel


@pytest.mark.parametrize("example_main", [
    single_agent,
    rag_memory,
    mcp_tool_use,
    streaming,
    human_in_loop,
    multi_agent,
    tool_composition,
    async_tools,
    structured_output,
    qwen_gateway,
    durable_restart,
    cost_otel,
], ids=[
    "single_agent",
    "rag_memory",
    "mcp_tool_use",
    "streaming",
    "human_in_loop",
    "multi_agent",
    "tool_composition",
    "async_tools",
    "structured_output",
    "qwen_gateway",
    "durable_restart",
    "cost_otel",
])
async def test_example_runs(example_main):
    await example_main()


async def test_conformance_runner_exits_zero():
    exit_code = await conformance_runner()
    assert exit_code == 0
