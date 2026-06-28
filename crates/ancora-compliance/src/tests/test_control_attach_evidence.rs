use crate::{ComplianceControl, Framework};
#[test]
fn attach_evidence_grows_list() {
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "Access", "desc");
    c.attach_evidence("ev-001");
    c.attach_evidence("ev-002");
    assert_eq!(c.evidence_count(), 2);
    assert!(c.evidence_ids.contains(&"ev-001".to_string()));
}
