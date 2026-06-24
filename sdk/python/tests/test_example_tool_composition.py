"""Test that the tool_composition example runs without error."""

from examples.tool_composition import main


async def test_tool_composition_example_runs():
    await main()
