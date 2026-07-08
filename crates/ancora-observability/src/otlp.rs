use serde::{Deserialize, Serialize};

use crate::span::{Span, SpanValue};

/// OTLP JSON attribute value union.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpAnyValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool_value: Option<bool>,
}

/// OTLP JSON key-value pair.
#[derive(Debug, Serialize, Deserialize)]
pub struct OtlpKeyValue {
    pub key: String,
    pub value: OtlpAnyValue,
}

/// OTLP JSON span (simplified subset).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpSpan {
    pub name: String,
    pub attributes: Vec<OtlpKeyValue>,
}

/// OTLP JSON scope spans wrapper.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpScopeSpans {
    pub spans: Vec<OtlpSpan>,
}

/// OTLP JSON resource spans wrapper.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpResourceSpans {
    pub scope_spans: Vec<OtlpScopeSpans>,
}

/// Top-level OTLP export traces request body.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtlpExportRequest {
    pub resource_spans: Vec<OtlpResourceSpans>,
}

fn span_value_to_otlp(sv: &SpanValue) -> OtlpAnyValue {
    match sv {
        SpanValue::String(s) => OtlpAnyValue {
            string_value: Some(s.clone()),
            int_value: None,
            double_value: None,
            bool_value: None,
        },
        SpanValue::Int(n) => OtlpAnyValue {
            string_value: None,
            int_value: Some(*n),
            double_value: None,
            bool_value: None,
        },
        SpanValue::Float(f) => OtlpAnyValue {
            string_value: None,
            int_value: None,
            double_value: Some(*f),
            bool_value: None,
        },
        SpanValue::Bool(b) => OtlpAnyValue {
            string_value: None,
            int_value: None,
            double_value: None,
            bool_value: Some(*b),
        },
    }
}

/// Convert a slice of spans into an OTLP export request body.
pub fn spans_to_otlp(spans: &[Span]) -> OtlpExportRequest {
    let otlp_spans: Vec<OtlpSpan> = spans
        .iter()
        .map(|s| OtlpSpan {
            name: s.name.clone(),
            attributes: s
                .attributes
                .iter()
                .map(|(k, v)| OtlpKeyValue {
                    key: k.clone(),
                    value: span_value_to_otlp(v),
                })
                .collect(),
        })
        .collect();

    OtlpExportRequest {
        resource_spans: vec![OtlpResourceSpans {
            scope_spans: vec![OtlpScopeSpans { spans: otlp_spans }],
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn spans_to_otlp_produces_resource_spans() {
        let spans = vec![Span::new("op").set("k", "v")];
        let req = spans_to_otlp(&spans);
        assert_eq!(req.resource_spans.len(), 1);
        let scope = &req.resource_spans[0].scope_spans[0];
        assert_eq!(scope.spans[0].name, "op");
    }

    #[test]
    fn spans_to_otlp_serializes_all_value_types() {
        use std::collections::HashMap;
        let spans = vec![Span::new("multi")
            .set("s", "text")
            .set("i", 42_i64)
            .set("f", 1.5_f64)
            .set("b", true)];
        let req = spans_to_otlp(&spans);
        let attrs: HashMap<String, _> = req.resource_spans[0].scope_spans[0].spans[0]
            .attributes
            .iter()
            .map(|kv| (kv.key.clone(), &kv.value))
            .collect();
        assert!(attrs["s"].string_value.is_some());
        assert!(attrs["i"].int_value.is_some());
        assert!(attrs["f"].double_value.is_some());
        assert!(attrs["b"].bool_value.is_some());
    }

    #[test]
    fn spans_to_otlp_with_empty_slice_produces_empty_scope_spans() {
        let req = spans_to_otlp(&[]);
        assert_eq!(req.resource_spans[0].scope_spans[0].spans.len(), 0);
    }

    #[test]
    fn spans_to_otlp_roundtrips_via_json() {
        let spans = vec![Span::new("op").set("k", "v")];
        let req = spans_to_otlp(&spans);
        let json = serde_json::to_string(&req).unwrap();
        let decoded: OtlpExportRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.resource_spans[0].scope_spans[0].spans[0].name, "op");
    }
}
