"""Detailed tests for each built-in conformance scenario."""

import ancora
from ancora.conformance import ConformanceSuite
from ancora.scenarios import register_builtin_scenarios


async def _suite():
    s = ConformanceSuite()
    register_builtin_scenarios(s)
    return s


async def test_tool_wire_format_scenario_passes():
    suite = await _suite()
    rt = ancora.Runtime()
    result = await suite.run_scenario("tool_wire_format", rt)
    rt.free()
    assert result is True


async def test_multi_run_isolation_scenario_passes():
    suite = await _suite()
    rt = ancora.Runtime()
    result = await suite.run_scenario("multi_run_isolation", rt)
    rt.free()
    assert result is True


async def test_stream_event_types_scenario_passes():
    suite = await _suite()
    rt = ancora.Runtime()
    result = await suite.run_scenario("stream_event_types", rt)
    rt.free()
    assert result is True


async def test_no_scenario_names_are_duplicated():
    suite = await _suite()
    assert len(suite.names) == len(set(suite.names))


async def test_all_builtin_scenarios_are_known():
    expected = {
        "single_run",
        "human_in_loop",
        "multi_run_isolation",
        "spec_roundtrip",
        "tool_wire_format",
        "streaming_tokens",
        "memory_persistence",
        "event_count",
        "stream_event_types",
    }
    suite = await _suite()
    assert set(suite.names) == expected
