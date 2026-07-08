use crate::{
    report_to_csv, ComplianceControl, ComplianceReport, ControlRegistry, ControlStatus, Framework,
};
#[test]
fn report_csv_has_header_and_data() {
    let mut reg = ControlRegistry::new();
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c.set_status(ControlStatus::Compliant, 1);
    reg.register(c);
    let report = ComplianceReport::generate(&reg, &Framework::Soc2, "t1", 99);
    let csv = report_to_csv(&report);
    assert!(csv.contains("framework,tenant_id"));
    assert!(csv.contains("SOC 2"));
    assert!(csv.contains("t1"));
}
