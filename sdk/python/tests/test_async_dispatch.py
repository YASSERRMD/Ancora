"""Tests for async tool dispatch via ToolRegistry.adispatch."""

import pytest
from ancora.tools import ToolRegistry, tool


async def test_adispatch_sync_tool():
    @tool
    def add(a: int, b: int) -> int:
        return a + b

    reg = ToolRegistry()
    reg.register(add)
    result = await reg.adispatch("add", '{"a": 2, "b": 3}')
    assert result == 5


async def test_adispatch_async_tool():
    import asyncio

    @tool
    def fetch(url: str) -> str:
        async def inner():
            await asyncio.sleep(0)
            return f"content of {url}"
        return inner()

    reg = ToolRegistry()
    reg.register(fetch)
    result = await reg.adispatch("fetch", '{"url": "https://example.com"}')
    assert "example.com" in result


async def test_adispatch_unknown_raises():
    reg = ToolRegistry()
    with pytest.raises(KeyError):
        await reg.adispatch("missing", "{}")


async def test_adispatch_empty_registry():
    reg = ToolRegistry()
    with pytest.raises(KeyError):
        await reg.adispatch("any_name")
