"""Tests for examples.helpers utilities."""

import json

from examples.helpers import pretty_results, print_event


def test_pretty_results_all_pass():
    results = {"a": True, "b": True}
    s = pretty_results(results)
    assert "PASS" in s
    assert "2/2" in s


def test_pretty_results_mixed():
    results = {"ok": True, "bad": False}
    s = pretty_results(results)
    assert "PASS" in s
    assert "FAIL" in s
    assert "1/2" in s


def test_print_event_token(capsys):
    raw = b'{"kind":"token","run_id":"r","text":"hi"}'
    print_event(raw)
    captured = capsys.readouterr()
    assert "hi" in captured.out


def test_print_event_started(capsys):
    raw = b'{"kind":"started","run_id":"abc123xyz"}'
    print_event(raw)
    captured = capsys.readouterr()
    assert "started" in captured.out


def test_print_event_completed(capsys):
    raw = b'{"kind":"completed","run_id":"r"}'
    print_event(raw)
    captured = capsys.readouterr()
    assert "completed" in captured.out


def test_print_event_resumed(capsys):
    raw = b'{"kind":"resumed","run_id":"r","decision":"yes"}'
    print_event(raw)
    captured = capsys.readouterr()
    assert "resumed" in captured.out
    assert "yes" in captured.out
