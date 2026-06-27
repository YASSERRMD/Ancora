"""Test that the qwen_gateway example runs without error."""

from examples.qwen_gateway import main


async def test_qwen_gateway_example_runs():
    await main()
