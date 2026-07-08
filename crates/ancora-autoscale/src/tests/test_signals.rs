#[cfg(test)]
mod tests {
    use crate::decision::ScaleDecision;
    use crate::signals::ScaleSignal;

    #[test]
    fn scale_up_signal_has_correct_desired() {
        let s = ScaleSignal::from_decision(ScaleDecision::ScaleUp { by: 3 }, 5, 10, 0.9);
        assert_eq!(s.desired_workers, 8);
        assert!(s.decision.is_scale_up());
    }

    #[test]
    fn scale_down_signal_has_correct_desired() {
        let s = ScaleSignal::from_decision(ScaleDecision::ScaleDown { by: 2 }, 6, 0, 0.1);
        assert_eq!(s.desired_workers, 4);
        assert!(s.decision.is_scale_down());
    }

    #[test]
    fn noop_signal_desired_equals_current() {
        let s = ScaleSignal::from_decision(
            ScaleDecision::NoOp {
                reason: "ok".to_string(),
            },
            4,
            2,
            0.5,
        );
        assert_eq!(s.desired_workers, 4);
        assert!(s.decision.is_noop());
    }

    #[test]
    fn signal_has_timestamp() {
        let s = ScaleSignal::from_decision(ScaleDecision::ScaleUp { by: 1 }, 2, 5, 0.8);
        assert!(s.ts <= chrono::Utc::now());
    }
}
