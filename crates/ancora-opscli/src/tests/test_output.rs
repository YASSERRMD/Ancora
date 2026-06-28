#[cfg(test)]
mod tests {
    use crate::output::{render, OutputFormat};
    use serde_json::json;

    #[test]
    fn json_output_valid() {
        let v = json!({"run_id": "r1", "status": "Running"});
        let out = render(&v, &OutputFormat::Json);
        assert!(out.contains("run_id"));
        assert!(out.contains("Running"));
    }

    #[test]
    fn table_output_contains_key_value() {
        let v = json!({"run_id": "r1", "status": "Running"});
        let out = render(&v, &OutputFormat::Table);
        assert!(out.contains("run_id"));
    }

    #[test]
    fn table_output_for_array() {
        let v = json!([{"id": "1"}, {"id": "2"}]);
        let out = render(&v, &OutputFormat::Table);
        assert!(out.contains("id=1"));
        assert!(out.contains("id=2"));
    }
}
