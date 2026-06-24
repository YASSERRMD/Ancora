"""End-to-end test: full conformance suite is green with zero failures."""

import ancora
from ancora import ConformanceSuite, register_builtin_scenarios


async def test_conformance_suite_all_green():
    suite = ConformanceSuite()
    register_builtin_scenarios(suite)
    rt = ancora.Runtime()
    results = await suite.run_all(rt)
    rt.free()

    failing = suite.failed(results)
    summary = suite.summary(results)

    assert failing == [], f"Conformance failures:\n{summary}"
    assert all(results.values()), f"Not all scenarios passed:\n{summary}"
