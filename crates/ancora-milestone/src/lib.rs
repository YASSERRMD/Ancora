/// Phase 200 milestone: advanced capabilities cross-suite verification.
///
/// This crate does not ship production code -- it exists solely to run
/// cross-crate integration checks that verify the full advanced suite is green.
#[cfg(test)]
mod tests {
    mod advanced_determinism;
    mod advanced_examples_run;
    mod advanced_parity;
    mod airgap_all;
    mod baselines_recorded;
    mod behavior_evals_green;
    mod bench_gate;
    mod cross_crate_smoke;
    mod docs_link_check;
    mod full_advanced_suite;
    mod guardrail_chain;
    mod parity_canonical;
    mod planning_chain;
    mod preset_catalog_link;
    mod preset_suite;
    mod reason_chain;
    mod red_team_green;
    mod redteam_regression;
    mod routing_chain;
    mod version_alignment;
}
