"""Tests for ConformanceSuite class internals."""

import pytest

import ancora
from ancora.conformance import ConformanceSuite


def test_empty_suite_has_no_names():
    suite = ConformanceSuite()
    assert suite.names == []


def test_register_adds_scenario():
    suite = ConformanceSuite()
    async def my_scenario(rt):
        return True
    suite.register("my", my_scenario)
    assert "my" in suite.names


def test_names_preserve_order():
    suite = ConformanceSuite()
    for name in ["alpha", "beta", "gamma"]:
        suite.register(name, lambda rt: True)
    assert suite.names == ["alpha", "beta", "gamma"]


async def test_run_scenario_returns_true_on_pass():
    suite = ConformanceSuite()
    async def always_pass(rt):
        return True
    suite.register("pass", always_pass)
    rt = ancora.Runtime()
    assert await suite.run_scenario("pass", rt) is True
    rt.free()


async def test_run_scenario_returns_false_on_fail():
    suite = ConformanceSuite()
    async def always_fail(rt):
        return False
    suite.register("fail", always_fail)
    rt = ancora.Runtime()
    assert await suite.run_scenario("fail", rt) is False
    rt.free()


async def test_run_scenario_catches_exception():
    suite = ConformanceSuite()
    async def raises(rt):
        raise RuntimeError("oops")
    suite.register("raises", raises)
    rt = ancora.Runtime()
    result = await suite.run_scenario("raises", rt)
    assert result is False
    rt.free()


async def test_run_all_returns_dict():
    suite = ConformanceSuite()
    suite.register("a", lambda rt: True)
    suite.register("b", lambda rt: True)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert isinstance(results, dict)
    assert set(results.keys()) == {"a", "b"}
    rt.free()


def test_run_scenario_unknown_raises():
    suite = ConformanceSuite()
    rt = ancora.Runtime()
    import asyncio
    with pytest.raises(KeyError):
        asyncio.get_event_loop().run_until_complete(suite.run_scenario("nope", rt))
    rt.free()
