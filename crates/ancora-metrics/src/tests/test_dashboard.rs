#[cfg(test)]
mod tests {
    use crate::dashboard::grafana_dashboard;

    #[test]
    fn dashboard_json_valid() {
        let d = grafana_dashboard();
        assert_eq!(d["title"], "Ancora SLO Dashboard");
        assert_eq!(d["schemaVersion"], 36);
    }

    #[test]
    fn dashboard_has_panels() {
        let d = grafana_dashboard();
        let panels = d["panels"].as_array().unwrap();
        assert!(!panels.is_empty());
    }

    #[test]
    fn dashboard_has_tenant_template() {
        let d = grafana_dashboard();
        let templates = d["templating"]["list"].as_array().unwrap();
        assert!(templates.iter().any(|t| t["name"] == "tenant"));
    }

    #[test]
    fn dashboard_serializes_to_json_string() {
        let d = grafana_dashboard();
        let s = serde_json::to_string_pretty(&d).unwrap();
        assert!(s.contains("ancora"));
    }
}
