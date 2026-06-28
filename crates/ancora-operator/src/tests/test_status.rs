#[cfg(test)]
mod tests {
    use crate::crd::cluster::ConditionStatus;
    use crate::status::{degraded_condition, make_condition, ready_condition};

    #[test]
    fn ready_condition_true() {
        let c = ready_condition(true, "all good");
        assert_eq!(c.condition_type, "Ready");
        assert_eq!(c.status, ConditionStatus::True);
    }

    #[test]
    fn ready_condition_false() {
        let c = ready_condition(false, "not ready");
        assert_eq!(c.status, ConditionStatus::False);
    }

    #[test]
    fn degraded_condition_set() {
        let c = degraded_condition("store unavailable");
        assert_eq!(c.condition_type, "Degraded");
        assert_eq!(c.status, ConditionStatus::True);
        assert!(c.message.contains("store"));
    }

    #[test]
    fn make_condition_custom() {
        let c = make_condition("Syncing", ConditionStatus::Unknown, "in progress");
        assert_eq!(c.condition_type, "Syncing");
        assert_eq!(c.status, ConditionStatus::Unknown);
    }
}
