pub mod access_control;
pub mod airgap;
pub mod cli;
pub mod fetch;
pub mod mirror;
pub mod publish;
pub mod search;
pub mod service;
pub mod signature;
pub mod versioning;

#[cfg(test)]
mod tests {
    mod test_access_control;
    mod test_airgap_offline;
    mod test_cli;
    mod test_mirror_syncs;
    mod test_publish_fetch;
    mod test_search_results;
    mod test_unsigned_rejected;
    mod test_version_listing;
}
