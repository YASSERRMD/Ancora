"""Test that the human_in_loop example runs without error."""

from examples.human_in_loop import main


async def test_human_in_loop_example_runs():
    await main()
