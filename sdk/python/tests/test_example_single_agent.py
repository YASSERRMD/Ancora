"""Test that the single_agent example runs without error."""

from examples.single_agent import main


async def test_single_agent_example_runs():
    await main()
