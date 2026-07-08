"""Phase 143 conformance task 7: conformance suite passes."""

import pytest
import ancora


@pytest.fixture
def suite():
    s = ancora.ConformanceSuite()
    ancora.register_builtin_scenarios(s)
    return s


@pytest.mark.asyncio
async def test_conf_suite_run_all_returns_dict(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert isinstance(results, dict)
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_has_at_least_one_scenario(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert len(results) >= 1
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_all_keys_are_strings(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert all(isinstance(k, str) for k in results.keys())
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_all_values_are_bool(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert all(isinstance(v, bool) for v in results.values())
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_contains_single_agent_scenario(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert any("single" in k.lower() or "agent" in k.lower() for k in results)
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_contains_verifier_scenario(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert any("verif" in k.lower() or "multi" in k.lower() for k in results)
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_all_scenarios_pass(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    failed = [k for k, v in results.items() if not v]
    assert failed == [], f"Failing scenarios: {failed}"
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_second_run_consistent(suite):
    rt = ancora.Runtime()
    results1 = await suite.run_all(rt)
    results2 = await suite.run_all(rt)
    assert set(results1.keys()) == set(results2.keys())
    rt.free()


@pytest.mark.asyncio
async def test_conf_suite_new_instance_per_runtime():
    rt1 = ancora.Runtime()
    rt2 = ancora.Runtime()
    suite1 = ancora.ConformanceSuite()
    ancora.register_builtin_scenarios(suite1)
    suite2 = ancora.ConformanceSuite()
    ancora.register_builtin_scenarios(suite2)
    r1 = await suite1.run_all(rt1)
    r2 = await suite2.run_all(rt2)
    assert set(r1.keys()) == set(r2.keys())
    rt1.free()
    rt2.free()


@pytest.mark.asyncio
async def test_conf_suite_scenario_names_non_empty(suite):
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    assert all(k != "" for k in results.keys())
    rt.free()
