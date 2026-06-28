use crate::control::ComplianceControl;
use crate::report::ComplianceReport;

pub fn report_to_csv(report: &ComplianceReport) -> String {
    format!(
        "framework,tenant_id,generated_tick,total,compliant,non_compliant,partially_compliant,not_assessed,not_applicable,compliance_rate\n{},{},{},{},{},{},{},{},{},{:.4}\n",
        report.framework,
        report.tenant_id,
        report.generated_tick,
        report.total_controls,
        report.compliant,
        report.non_compliant,
        report.partially_compliant,
        report.not_assessed,
        report.not_applicable,
        report.compliance_rate(),
    )
}

pub fn controls_to_csv(controls: &[&ComplianceControl]) -> String {
    let mut out = String::from("id,framework,title,status,evidence_count,assessed_tick\n");
    for c in controls {
        out.push_str(&format!(
            "{},{},{},{},{},{}\n",
            c.id,
            c.framework,
            c.title,
            c.status,
            c.evidence_count(),
            c.assessed_tick.map(|t| t.to_string()).unwrap_or_default(),
        ));
    }
    out
}
