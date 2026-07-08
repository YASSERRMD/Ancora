pub mod announcement;
pub mod apps_status;
pub mod changelog;
pub mod e2e_status;
pub mod feature_matrix;
pub mod index;
pub mod itk_status;
pub mod limitations;
pub mod quickstart;
pub mod readiness;
pub mod registry_links;
pub mod suite_status;
pub mod trust_summary;
pub mod upgrade_notes;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod test_apps_run;
    mod test_docs_link_check;
    mod test_e2e_green;
    mod test_eco_baselines;
    mod test_ecosystem_suite_green;
    mod test_itk_green;
}
