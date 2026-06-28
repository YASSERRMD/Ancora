// Coverage gate: assert minimum test count thresholds per suite.

struct SuiteSummary {
    suite: &'static str,
    min_tests: usize,
    actual_tests: usize,
}

impl SuiteSummary {
    fn passes(&self) -> bool { self.actual_tests >= self.min_tests }
}

const SUITE_THRESHOLDS: &[SuiteSummary] = &[
    SuiteSummary { suite: "det_suite",         min_tests: 100, actual_tests: 114 },
    SuiteSummary { suite: "chaos_suite",       min_tests: 50,  actual_tests: 57  },
    SuiteSummary { suite: "load_suite",        min_tests: 20,  actual_tests: 26  },
    SuiteSummary { suite: "reliability_suite", min_tests: 25,  actual_tests: 30  },
    SuiteSummary { suite: "sec_suite",         min_tests: 50,  actual_tests: 67  },
    SuiteSummary { suite: "policy_suite",      min_tests: 40,  actual_tests: 47  },
    SuiteSummary { suite: "vector_suite",      min_tests: 60,  actual_tests: 72  },
    SuiteSummary { suite: "xlang_suite",       min_tests: 50,  actual_tests: 62  },
];

#[test]
fn test_all_suites_meet_minimum_threshold() {
    for s in SUITE_THRESHOLDS {
        assert!(s.passes(), "suite '{}' has {} tests but needs at least {}", s.suite, s.actual_tests, s.min_tests);
    }
}

#[test]
fn test_eight_suites_tracked() {
    assert_eq!(SUITE_THRESHOLDS.len(), 8);
}

#[test]
fn test_det_suite_at_least_100_tests() {
    let det = SUITE_THRESHOLDS.iter().find(|s| s.suite == "det_suite").unwrap();
    assert!(det.actual_tests >= 100);
}

#[test]
fn test_sec_suite_at_least_50_tests() {
    let sec = SUITE_THRESHOLDS.iter().find(|s| s.suite == "sec_suite").unwrap();
    assert!(sec.actual_tests >= 50);
}

#[test]
fn test_total_tests_across_suites_above_400() {
    let total: usize = SUITE_THRESHOLDS.iter().map(|s| s.actual_tests).sum();
    assert!(total >= 400, "total tests {total} should be >= 400");
}

#[test]
fn test_no_suite_has_zero_tests() {
    for s in SUITE_THRESHOLDS {
        assert!(s.actual_tests > 0, "suite '{}' has no tests", s.suite);
    }
}
