use crate::attribute::{AttributeSet, AttributeValue};

#[derive(Debug, Clone)]
pub enum Condition {
    Eq(String, AttributeValue),
    NotEq(String, AttributeValue),
    GreaterThan(String, i64),
    LessThan(String, i64),
    Contains(String, String),
    Exists(String),
    NotExists(String),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn evaluate(&self, subject: &AttributeSet, resource: &AttributeSet, env: &AttributeSet) -> bool {
        let lookup = |key: &str| -> Option<&AttributeValue> {
            subject.get(key).or_else(|| resource.get(key)).or_else(|| env.get(key))
        };
        match self {
            Condition::Eq(k, v) => lookup(k).map(|a| a == v).unwrap_or(false),
            Condition::NotEq(k, v) => lookup(k).map(|a| a != v).unwrap_or(true),
            Condition::GreaterThan(k, n) => lookup(k).and_then(|a| a.as_int()).map(|i| i > *n).unwrap_or(false),
            Condition::LessThan(k, n) => lookup(k).and_then(|a| a.as_int()).map(|i| i < *n).unwrap_or(false),
            Condition::Contains(k, v) => lookup(k).map(|a| a.contains(v)).unwrap_or(false),
            Condition::Exists(k) => lookup(k).is_some(),
            Condition::NotExists(k) => lookup(k).is_none(),
            Condition::And(a, b) => a.evaluate(subject, resource, env) && b.evaluate(subject, resource, env),
            Condition::Or(a, b) => a.evaluate(subject, resource, env) || b.evaluate(subject, resource, env),
            Condition::Not(c) => !c.evaluate(subject, resource, env),
        }
    }
}
