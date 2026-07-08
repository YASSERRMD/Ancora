use crate::{controls_to_csv, ComplianceControl, Framework};
#[test]
fn controls_csv_has_header_and_data() {
    let c = ComplianceControl::new("CC6.1", Framework::Soc2, "Access", "desc");
    let csv = controls_to_csv(&[&c]);
    assert!(csv.starts_with("id,framework,title,status"));
    assert!(csv.contains("CC6.1"));
    assert!(csv.contains("SOC 2"));
    assert!(csv.contains("NOT_ASSESSED"));
}
