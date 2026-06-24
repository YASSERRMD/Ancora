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
        Self { name: name.into(), attributes: HashMap::new() }
    }

    pub fn set(mut self, key: impl Into<String>, value: impl Into<SpanValue>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

impl From<String> for SpanValue {
    fn from(s: String) -> Self { SpanValue::String(s) }
}
impl From<&str> for SpanValue {
    fn from(s: &str) -> Self { SpanValue::String(s.to_owned()) }
}
impl From<i64> for SpanValue {
    fn from(n: i64) -> Self { SpanValue::Int(n) }
}
impl From<u64> for SpanValue {
    fn from(n: u64) -> Self { SpanValue::Int(n as i64) }
}
impl From<f64> for SpanValue {
    fn from(f: f64) -> Self { SpanValue::Float(f) }
}
impl From<bool> for SpanValue {
    fn from(b: bool) -> Self { SpanValue::Bool(b) }
}
