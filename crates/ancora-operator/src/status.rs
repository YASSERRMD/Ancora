use crate::crd::cluster::{Condition, ConditionStatus};
use chrono::Utc;

pub fn make_condition(ctype: &str, status: ConditionStatus, message: &str) -> Condition {
    Condition {
        condition_type: ctype.to_string(),
        status,
        message: message.to_string(),
        last_transition: Utc::now().to_rfc3339(),
    }
}

pub fn ready_condition(ready: bool, message: &str) -> Condition {
    make_condition(
        "Ready",
        if ready { ConditionStatus::True } else { ConditionStatus::False },
        message,
    )
}

pub fn degraded_condition(reason: &str) -> Condition {
    make_condition("Degraded", ConditionStatus::True, reason)
}
