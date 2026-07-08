use std::collections::HashMap;

/// An OTel-compatible span with a name and key-value attributes.
#[derive(Debug, Clone)]
pub struct Span {
    pub name: String,
    pub attributes: HashMap<String, SpanValue>,
}

/// A typed attribute value (subset of OTel attribute types).
#[derive(Debug, Clone, PartialEq)]
pub enum SpanValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Span {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: HashMap::new(),
        }
    }

    pub fn set(mut self, key: impl Into<String>, value: impl Into<SpanValue>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: &str) -> Option<&SpanValue> {
        self.attributes.get(key)
    }
}

impl From<String> for SpanValue {
    fn from(s: String) -> Self {
        SpanValue::String(s)
    }
}
impl From<&str> for SpanValue {
    fn from(s: &str) -> Self {
        SpanValue::String(s.to_owned())
    }
}
impl From<i64> for SpanValue {
    fn from(n: i64) -> Self {
        SpanValue::Int(n)
    }
}
impl From<u64> for SpanValue {
    fn from(n: u64) -> Self {
        SpanValue::Int(n as i64)
    }
}
impl From<f64> for SpanValue {
    fn from(f: f64) -> Self {
        SpanValue::Float(f)
    }
}
impl From<bool> for SpanValue {
    fn from(b: bool) -> Self {
        SpanValue::Bool(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_value_from_str() {
        assert_eq!(SpanValue::from("hello"), SpanValue::String("hello".into()));
    }

    #[test]
    fn span_value_from_i64() {
        assert_eq!(SpanValue::from(42_i64), SpanValue::Int(42));
    }

    #[test]
    fn span_value_from_u64_casts_to_int() {
        assert_eq!(SpanValue::from(99_u64), SpanValue::Int(99));
    }

    #[test]
    fn span_value_from_f64() {
        assert_eq!(SpanValue::from(1.5_f64), SpanValue::Float(1.5));
    }

    #[test]
    fn span_value_from_bool() {
        assert_eq!(SpanValue::from(true), SpanValue::Bool(true));
    }

    #[test]
    fn span_get_returns_attribute() {
        let span = Span::new("test").set("key", "val");
        assert_eq!(span.get("key"), Some(&SpanValue::String("val".into())));
        assert_eq!(span.get("missing"), None);
    }

    #[test]
    fn span_builder_chains_multiple_attributes() {
        let span = Span::new("op").set("a", 1_i64).set("b", true);
        assert_eq!(span.get("a"), Some(&SpanValue::Int(1)));
        assert_eq!(span.get("b"), Some(&SpanValue::Bool(true)));
    }
}
