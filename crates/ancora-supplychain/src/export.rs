use crate::component::Component;
use crate::sbom::Sbom;

pub fn sbom_to_csv(sbom: &Sbom) -> String {
    let mut lines = vec!["id,name,version,kind,license,supplier,digest".to_string()];
    for c in &sbom.components {
        lines.push(format!("{},{},{},{},{},{},{}", c.id, c.name, c.version, c.kind, c.license, c.supplier, c.digest));
    }
    lines.join("\n")
}

pub fn components_to_csv(components: &[&Component]) -> String {
    let mut lines = vec!["id,name,version,kind,license,supplier".to_string()];
    for c in components {
        lines.push(format!("{},{},{},{},{},{}", c.id, c.name, c.version, c.kind, c.license, c.supplier));
    }
    lines.join("\n")
}

pub fn sbom_to_summary(sbom: &Sbom) -> String {
    format!(
        "SBOM {} ({}) - {} components",
        sbom.id, sbom.tenant_id, sbom.component_count()
    )
}
