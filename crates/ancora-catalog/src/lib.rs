pub mod connector_entry;
pub mod entry_schema;
pub mod index;
pub mod install;
pub mod metadata;
pub mod provider_entry;
pub mod search;
pub mod signing;
pub mod tool_entry;
pub mod validation;
pub mod vectorstore_entry;

#[cfg(test)]
mod tests {
    mod test_entry_validates;
    mod test_index_loads;
    mod test_install_adds;
    mod test_invalid_rejected;
    mod test_license_recorded;
    mod test_search_by_tag;
    mod test_signed_catalog;
}
