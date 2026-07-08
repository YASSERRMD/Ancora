use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    String(String),
    Integer(i64),
    Bool(bool),
    List(Vec<String>),
}

impl AttributeValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            AttributeValue::Integer(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttributeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn contains(&self, val: &str) -> bool {
        match self {
            AttributeValue::List(l) => l.iter().any(|s| s == val),
            _ => false,
        }
    }
}

impl From<&str> for AttributeValue {
    fn from(s: &str) -> Self {
        AttributeValue::String(s.to_string())
    }
}
impl From<String> for AttributeValue {
    fn from(s: String) -> Self {
        AttributeValue::String(s)
    }
}
impl From<i64> for AttributeValue {
    fn from(i: i64) -> Self {
        AttributeValue::Integer(i)
    }
}
impl From<bool> for AttributeValue {
    fn from(b: bool) -> Self {
        AttributeValue::Bool(b)
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttributeSet {
    pub attrs: HashMap<String, AttributeValue>,
}

impl AttributeSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<AttributeValue>) {
        self.attrs.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&AttributeValue> {
        self.attrs.get(key)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
}
