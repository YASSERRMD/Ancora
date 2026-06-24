"""Tests for registering and running custom conformance scenarios."""

import ancora
from ancora.conformance import ConformanceSuite
from ancora.scenarios import register_builtin_scenarios


async def test_custom_scenario_added_after_builtins():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    initial_count = len(suite.names)

    async def custom(rt):
        return True

    suite.register("custom_check", custom)
    assert len(suite.names) == initial_count + 1
    assert suite.names[-1] == "custom_check"


async def test_custom_scenario_runs_in_run_all():
    suite = ConformanceSuite()
    ran = []

    async def my_check(rt):
        ran.append("my_check")
        return True

    suite.register("my_check", my_check)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    assert "my_check" in ran
    assert results["my_check"] is True


async def test_custom_scenario_overrides_existing():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    # Override single_run to always fail
    suite.register("single_run", lambda rt: False)
    rt = ancora.Runtime()
    result = await suite.run_scenario("single_run", rt)
    rt.free()
    assert result is False


async def test_run_all_with_mixed_custom_and_builtins():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    suite.register("extra_pass", lambda rt: True)
    suite.register("extra_fail", lambda rt: False)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    assert results["extra_pass"] is True
    assert results["extra_fail"] is False
    assert len(suite.failed(results)) == 1
    assert "extra_fail" in suite.failed(results)
