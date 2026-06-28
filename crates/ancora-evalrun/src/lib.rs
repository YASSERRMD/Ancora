/// ancora-evalrun: Eval runs at scale with statistics, clustering, comparison, and reports.

pub mod executor;
pub mod rollout;
pub mod passatk;
pub mod aggregate;
pub mod breakdown;
pub mod cluster;
pub mod cost_latency;
pub mod persistence;
pub mod compare;
pub mod report;
pub mod cli;

#[cfg(test)]
mod tests {
    mod test_eval_run;
    mod test_pass_power_k;
    mod test_confidence_intervals;
    mod test_failure_clustering;
    mod test_run_comparison;
    mod test_report_generated;
    mod test_eval_seed;
    mod test_cli_runs_eval;
}
