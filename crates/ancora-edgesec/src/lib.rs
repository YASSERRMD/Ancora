pub mod identity;
pub mod boot;
pub mod attestation;
pub mod storage;
pub mod tamper;
pub mod report;
pub mod revocation;
pub mod airgap_proof;
pub mod egress;

#[cfg(test)]
mod tests {
    mod test_identity;
    mod test_attestation;
    mod test_storage;
    mod test_tamper;
    mod test_report;
    mod test_revocation;
    mod test_airgap_proof;
    mod test_egress;
}
