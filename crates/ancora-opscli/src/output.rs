use serde::Serialize;

/// Output format selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Table,
}

/// Render a serializable value as JSON or a simple table.
pub fn render<T: Serialize>(value: &T, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(value).unwrap_or_default(),
        OutputFormat::Table => table_from_json(value),
    }
}

fn table_from_json<T: Serialize>(value: &T) -> String {
    let v = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
    match &v {
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(json_object_row)
            .collect::<Vec<_>>()
            .join("\n"),
        serde_json::Value::Object(_) => json_object_row(&v),
        _ => v.to_string(),
    }
}

fn json_object_row(v: &serde_json::Value) -> String {
    if let serde_json::Value::Object(m) = v {
        m.iter()
            .map(|(k, val)| format!("{k}={}", val.as_str().unwrap_or(&val.to_string())))
            .collect::<Vec<_>>()
            .join("  ")
    } else {
        v.to_string()
    }
}
