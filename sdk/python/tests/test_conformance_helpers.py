"""Tests for ConformanceSuite.passed, failed, and summary helpers."""

import ancora
from ancora.conformance import ConformanceSuite


async def test_passed_returns_passing_names():
    suite = ConformanceSuite()
    suite.register("ok", lambda rt: True)
    suite.register("bad", lambda rt: False)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    assert suite.passed(results) == ["ok"]


async def test_failed_returns_failing_names():
    suite = ConformanceSuite()
    suite.register("ok", lambda rt: True)
    suite.register("bad", lambda rt: False)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    assert suite.failed(results) == ["bad"]


async def test_summary_contains_pass_fail_labels():
    suite = ConformanceSuite()
    suite.register("good", lambda rt: True)
    suite.register("evil", lambda rt: False)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    s = suite.summary(results)
    assert "PASS" in s
    assert "FAIL" in s
    assert "1/2" in s


async def test_all_passed_summary():
    suite = ConformanceSuite()
    suite.register("a", lambda rt: True)
    suite.register("b", lambda rt: True)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    assert suite.failed(results) == []
    assert "2/2" in suite.summary(results)
