use crate::attribute::AttributeValue;
use crate::condition::Condition;
use crate::policy::{Effect, Policy};

pub fn allow_if_department(dept: impl Into<String>) -> Policy {
    let dept = dept.into();
    Policy::new(
        format!("allow-dept-{dept}"),
        Effect::Allow,
        vec!["*".into()],
        Condition::Eq("department".into(), AttributeValue::String(dept)),
    )
}

pub fn deny_if_blocked() -> Policy {
    Policy::new(
        "deny-blocked",
        Effect::Deny,
        vec!["*".into()],
        Condition::Eq("blocked".into(), AttributeValue::Bool(true)),
    )
    .with_priority(1)
}

pub fn allow_if_classification_at_most(max_level: i64) -> Policy {
    Policy::new(
        format!("allow-class-{max_level}"),
        Effect::Allow,
        vec!["read".into()],
        Condition::LessThan("classification_level".into(), max_level + 1),
    )
}
