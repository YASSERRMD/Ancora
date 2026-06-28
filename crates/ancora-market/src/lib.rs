/// ancora-market: Marketplace trust metadata for the Ancora agent framework.
///
/// Shared extensions carry signed, scanned, licensed, residency-aware trust
/// metadata enforced on install.

pub mod metadata_schema;
pub mod identity;
pub mod badge;
pub mod security_scan;
pub mod license;
pub mod residency;
pub mod versioning;
pub mod dependency;
pub mod trust_score;
pub mod policy;

#[cfg(test)]
mod tests {
    mod test_metadata_validates;
    mod test_signature_verifies;
    mod test_badge_reflected;
    mod test_security_scan;
    mod test_license_required;
    mod test_residency_enforced;
    mod test_trust_score;
    mod test_low_trust_blocked;
}
