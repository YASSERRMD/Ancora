"""Test that the cost_otel example runs without error."""

from examples.cost_otel import main, estimate_tokens, Span


async def test_cost_otel_example_runs():
    await main()


def test_estimate_tokens_minimum_one():
    assert estimate_tokens(b"") == 1


def test_estimate_tokens_four_bytes():
    assert estimate_tokens(b"abcd") == 1


def test_estimate_tokens_larger():
    payload = b"x" * 100
    assert estimate_tokens(payload) == 25


def test_span_set_and_end_does_not_raise(capsys):
    s = Span("test.span")
    s.set_attribute("key", "val")
    s.end()
    out = capsys.readouterr().out
    assert "test.span" in out
    assert "key" in out
