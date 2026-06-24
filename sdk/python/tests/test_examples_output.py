"""Tests verifying example outputs are correct."""

import asyncio

import ancora
from examples.rag_memory import retrieve, summarize
from examples.mcp_tool_use import search_web, get_weather, calculate
from examples.tool_composition import tokenize, count_tokens, word_frequencies, registry


def test_rag_retrieve_returns_docs():
    result = retrieve.call_with_kwargs(query="Ancora")
    assert "doc1" in result
    assert "Ancora" in result


def test_rag_summarize_truncates():
    long_text = " ".join([f"word{i}" for i in range(20)])
    summary = summarize.call_with_kwargs(text=long_text)
    assert "..." in summary


def test_mcp_search_returns_list():
    results = search_web.call_with_kwargs(query="test", limit=3)
    assert isinstance(results, list)
    assert len(results) == 3


def test_mcp_get_weather_contains_location():
    result = get_weather.call_with_kwargs(location="Paris")
    assert "Paris" in result


def test_mcp_calculate_arithmetic():
    result = calculate.call_with_kwargs(expression="6 * 7")
    assert result == 42.0


def test_composition_tokenize_splits():
    tokens = registry.dispatch("tokenize", '{"text": "hello world"}')
    assert tokens == ["hello", "world"]


def test_composition_count_tokens():
    count = registry.dispatch("count_tokens", '{"text": "a b c d"}')
    assert count == 4


def test_composition_word_frequencies():
    freqs = registry.dispatch("word_frequencies", '{"text": "a b a c a b"}')
    assert freqs == {"a": 3, "b": 2, "c": 1}
