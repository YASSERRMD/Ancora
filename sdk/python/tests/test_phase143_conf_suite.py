"""Phase 143 conformance task 7: conformance suite passes."""

import pytest
import ancora


@pytest.mark.asyncio
async def test_conf_suite_run_all_returns_dict():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert isinstance(results, dict)
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_has_at_least_one_scenario():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert len(results) >= 1
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_all_keys_are_strings():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert all(isinstance(k, str) for k in results.keys())
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_all_values_are_bool():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert all(isinstance(v, bool) for v in results.values())
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_contains_single_agent_scenario():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert any("single" in k.lower() or "agent" in k.lower() for k in results)
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_contains_verifier_scenario():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert any("verif" in k.lower() or "multi" in k.lower() for k in results)
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_all_scenarios_pass():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    failed = [k for k, v in results.items() if not v]
    assert failed == [], f"Failing scenarios: {failed}"
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_second_run_consistent():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results1 = await suite.run_all(rt)
    results2 = await suite.run_all(rt)
    assert set(results1.keys()) == set(results2.keys())
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_new_instance_per_runtime():
    rt1 = ancora.Runtime()
    rt2 = ancora.Runtime()
    r1 = await ancora.ConformanceSuite().run_all(rt1)
    r2 = await ancora.ConformanceSuite().run_all(rt2)
    assert set(r1.keys()) == set(r2.keys())
    rt1.free()
    rt2.free()


@pytest.mark.asyncio
async def test_conf_suite_scenario_names_non_empty():
    rt = ancora.Runtime()
    suite = ancora.ConformanceSuite()
    results = await suite.run_all(rt)
    assert all(k != "" for k in results.keys())
    rt.free()
