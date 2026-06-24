"""Tests that conformance symbols are importable from the ancora namespace."""

from ancora import ConformanceSuite, register_builtin_scenarios
from ancora.conformance import CORE_FIXTURE


def test_conformance_suite_importable_from_ancora():
    assert ConformanceSuite is not None


def test_register_builtin_scenarios_importable_from_ancora():
    assert callable(register_builtin_scenarios)


def test_core_fixture_importable_from_conformance():
    assert isinstance(CORE_FIXTURE, dict)


def test_core_fixture_has_single_run():
    assert "single_run" in CORE_FIXTURE
    assert "event_kinds" in CORE_FIXTURE["single_run"]
    assert "event_count" in CORE_FIXTURE["single_run"]


def test_core_fixture_has_human_in_loop():
    assert "human_in_loop" in CORE_FIXTURE
    assert "resume_event_kinds" in CORE_FIXTURE["human_in_loop"]


def test_create_suite_and_register():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    assert len(suite.names) > 0
