"""Test that the async_tools example runs without error."""

from examples.async_tools import main


async def test_async_tools_example_runs():
    await main()
