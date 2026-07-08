pub mod aggregate;
pub mod breakdown;
pub mod cli;
pub mod cluster;
pub mod compare;
pub mod cost_latency;
/// ancora-evalrun: Eval runs at scale with statistics, clustering, comparison, and reports.
pub mod executor;
pub mod passatk;
pub mod persistence;
pub mod report;
pub mod rollout;

#[cfg(test)]
mod tests {
    mod test_cli_runs_eval;
    mod test_confidence_intervals;
    mod test_eval_run;
    mod test_eval_seed;
    mod test_failure_clustering;
    mod test_pass_power_k;
    mod test_report_generated;
    mod test_run_comparison;
}
