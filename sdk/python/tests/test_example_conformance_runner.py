"""Test that the conformance_runner example exits with code 0."""

from examples.conformance_runner import main


async def test_conformance_runner_exits_zero():
    exit_code = await main()
    assert exit_code == 0
