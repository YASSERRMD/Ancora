"""Test that the durable_restart example runs without error and replays events."""

from examples.durable_restart import main, RunJournal
from ancora import MemoryStore


async def test_durable_restart_example_runs():
    await main()


def test_run_journal_records_and_replays():
    mem = MemoryStore()
    journal = RunJournal(mem)
    journal.record_run("run-1")
    journal.append_event("run-1", b'{"kind":"started"}')
    journal.append_event("run-1", b'{"kind":"completed"}')

    events = journal.events_for_run("run-1")
    assert len(events) == 2
    assert journal.run_count() == 1


def test_run_journal_multiple_runs():
    mem = MemoryStore()
    journal = RunJournal(mem)
    journal.record_run("a")
    journal.record_run("b")
    assert journal.run_count() == 2
    assert journal.events_for_run("a") == []
