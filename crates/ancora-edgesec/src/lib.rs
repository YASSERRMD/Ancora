pub mod airgap_proof;
pub mod attestation;
pub mod boot;
pub mod egress;
pub mod identity;
pub mod report;
pub mod revocation;
pub mod storage;
pub mod tamper;

#[cfg(test)]
mod tests {
    mod test_airgap_proof;
    mod test_attestation;
    mod test_egress;
    mod test_identity;
    mod test_report;
    mod test_revocation;
    mod test_storage;
    mod test_tamper;
}
