"""Test that the rag_memory example runs without error."""

from examples.rag_memory import main


async def test_rag_memory_example_runs():
    await main()
