"""Tests that run all built-in conformance scenarios."""

import ancora
from ancora.conformance import ConformanceSuite
from ancora.scenarios import register_builtin_scenarios


async def test_run_all_scenarios_pass():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()
    failing = [name for name, passed in results.items() if not passed]
    assert failing == [], f"Failing scenarios: {failing}"


async def test_builtin_scenario_count():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    assert len(suite.names) == 9


async def test_single_run_scenario_passes():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    assert await suite.run_scenario("single_run", rt) is True
    rt.free()


async def test_human_in_loop_scenario_passes():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    assert await suite.run_scenario("human_in_loop", rt) is True
    rt.free()


async def test_spec_roundtrip_scenario_passes():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    assert await suite.run_scenario("spec_roundtrip", rt) is True
    rt.free()


async def test_streaming_tokens_scenario_passes():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    assert await suite.run_scenario("streaming_tokens", rt) is True
    rt.free()


async def test_memory_persistence_scenario_passes():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    assert await suite.run_scenario("memory_persistence", rt) is True
    rt.free()


async def test_event_count_scenario_passes():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    assert await suite.run_scenario("event_count", rt) is True
    rt.free()
