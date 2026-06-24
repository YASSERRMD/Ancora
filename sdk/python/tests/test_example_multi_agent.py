"""Test that the multi_agent example runs without error."""

from examples.multi_agent import main


async def test_multi_agent_example_runs():
    await main()
