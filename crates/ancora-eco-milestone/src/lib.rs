pub mod suite_status;
pub mod e2e_status;
pub mod apps_status;
pub mod itk_status;
pub mod feature_matrix;
pub mod limitations;
pub mod upgrade_notes;
pub mod changelog;
pub mod quickstart;
pub mod registry_links;
pub mod trust_summary;
pub mod announcement;
pub mod readiness;
pub mod index;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod test_ecosystem_suite_green;
    mod test_e2e_green;
    mod test_apps_run;
    mod test_itk_green;
    mod test_docs_link_check;
    mod test_eco_baselines;
}
