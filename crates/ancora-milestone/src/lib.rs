/// Phase 200 milestone: advanced capabilities cross-suite verification.
///
/// This crate does not ship production code -- it exists solely to run
/// cross-crate integration checks that verify the full advanced suite is green.
#[cfg(test)]
mod tests {
    mod full_advanced_suite;
    mod advanced_parity;
    mod advanced_determinism;
    mod red_team_green;
    mod behavior_evals_green;
    mod preset_catalog_link;
    mod advanced_examples_run;
    mod docs_link_check;
    mod baselines_recorded;
    mod version_alignment;
    mod airgap_all;
    mod cross_crate_smoke;
    mod bench_gate;
    mod preset_suite;
    mod redteam_regression;
    mod parity_canonical;
    mod guardrail_chain;
    mod reason_chain;
    mod planning_chain;
    mod routing_chain;
}
