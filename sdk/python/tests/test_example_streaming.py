"""Test that the streaming example runs without error."""

from examples.streaming import main


async def test_streaming_example_runs():
    await main()
