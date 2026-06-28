//! Offline eval runner.
//!
//! Aggregates results from all eval suites and produces a summary report.
//! All execution is local - no network calls are made.

use crate::coordination::CoordinationSuite;
use crate::cost_efficiency::CostEfficiencySuite;
use crate::long_context::LongContextSuite;
use crate::multilingual::MultilingualSuite;
use crate::rag_faithfulness::RagFaithfulnessSuite;
use crate::reasoning::ReasoningSuite;
use crate::routing::RoutingSuite;
use crate::safety::SafetySuite;
use crate::tool_use::ToolUseSuite;

/// Result for a single named suite.
#[derive(Debug, Clone)]
pub struct SuiteResult {
    pub name: String,
    pub passed: usize,
    pub total: usize,
}

impl SuiteResult {
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.passed as f64 / self.total as f64
    }

    pub fn all_passed(&self) -> bool {
        self.passed == self.total
    }
}

/// Aggregated report from running the full eval catalog.
#[derive(Debug)]
pub struct EvalReport {
    pub suite_results: Vec<SuiteResult>,
}

impl EvalReport {
    pub fn total_passed(&self) -> usize {
        self.suite_results.iter().map(|r| r.passed).sum()
    }

    pub fn total_cases(&self) -> usize {
        self.suite_results.iter().map(|r| r.total).sum()
    }

    pub fn overall_pass_rate(&self) -> f64 {
        let total = self.total_cases();
        if total == 0 {
            return 0.0;
        }
        self.total_passed() as f64 / total as f64
    }

    pub fn all_suites_passed(&self) -> bool {
        self.suite_results.iter().all(|r| r.all_passed())
    }
}

/// Run all default eval suites offline and return a report.
pub fn run_offline_eval() -> EvalReport {
    let mut results = Vec::new();

    // Tool-use suite
    let (p, t) = ToolUseSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "tool_use".into(), passed: p, total: t });

    // RAG faithfulness suite
    let (p, t) = RagFaithfulnessSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "rag_faithfulness".into(), passed: p, total: t });

    // Coordination suite
    let (p, t) = CoordinationSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "coordination".into(), passed: p, total: t });

    // Reasoning suite
    let (p, t) = ReasoningSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "reasoning".into(), passed: p, total: t });

    // Safety suite
    let (p, t) = SafetySuite::default_catalog().run_all();
    results.push(SuiteResult { name: "safety".into(), passed: p, total: t });

    // Routing suite
    let (p, t) = RoutingSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "routing".into(), passed: p, total: t });

    // Long-context suite
    let (p, t) = LongContextSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "long_context".into(), passed: p, total: t });

    // Multilingual suite
    let (p, t) = MultilingualSuite::default_catalog().run_all();
    results.push(SuiteResult { name: "multilingual".into(), passed: p, total: t });

    // Cost-efficiency suite
    let (p, t) = CostEfficiencySuite::default_catalog().run_all();
    results.push(SuiteResult { name: "cost_efficiency".into(), passed: p, total: t });

    EvalReport { suite_results: results }
}
