"""Test that the mcp_tool_use example runs without error."""

from examples.mcp_tool_use import main


async def test_mcp_tool_use_example_runs():
    await main()
