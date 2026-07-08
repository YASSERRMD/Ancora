pub mod badge;
pub mod dependency;
pub mod identity;
pub mod license;
/// ancora-market: Marketplace trust metadata for the Ancora agent framework.
///
/// Shared extensions carry signed, scanned, licensed, residency-aware trust
/// metadata enforced on install.
pub mod metadata_schema;
pub mod policy;
pub mod residency;
pub mod security_scan;
pub mod trust_score;
pub mod versioning;

#[cfg(test)]
mod tests {
    mod test_badge_reflected;
    mod test_license_required;
    mod test_low_trust_blocked;
    mod test_metadata_validates;
    mod test_residency_enforced;
    mod test_security_scan;
    mod test_signature_verifies;
    mod test_trust_score;
}
