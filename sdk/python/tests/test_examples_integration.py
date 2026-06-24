"""Integration tests verifying examples interact correctly with the runtime."""

import ancora
from examples.single_agent import main as single_agent_main
from examples.rag_memory import main as rag_memory_main
from examples.conformance_runner import main as conformance_main


async def test_single_agent_produces_events(capsys):
    await single_agent_main()
    captured = capsys.readouterr()
    assert "started run:" in captured.out
    assert "done." in captured.out


async def test_rag_memory_stores_keys(capsys):
    await rag_memory_main()
    captured = capsys.readouterr()
    assert "retrieved:" in captured.out
    assert "summary:" in captured.out
    assert "response:" in captured.out
    assert "memory keys:" in captured.out


async def test_conformance_runner_all_pass(capsys):
    exit_code = await conformance_main()
    captured = capsys.readouterr()
    assert exit_code == 0
    assert "9/9" in captured.out
