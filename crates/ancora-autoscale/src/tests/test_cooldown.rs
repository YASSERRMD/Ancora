#[cfg(test)]
mod tests {
    use crate::cooldown::Cooldown;

    #[test]
    fn can_scale_up_immediately_after_new() {
        let c = Cooldown::new(60, 120);
        assert!(c.can_scale_up());
    }

    #[test]
    fn cannot_scale_up_within_cooldown() {
        let mut c = Cooldown::new(3600, 3600);
        c.record_scale_up();
        assert!(!c.can_scale_up(), "still in cooldown");
    }

    #[test]
    fn cooldown_prevents_flapping() {
        let mut c = Cooldown::new(3600, 3600);
        c.record_scale_up();
        c.record_scale_down();
        assert!(!c.can_scale_up());
        assert!(!c.can_scale_down());
    }

    #[test]
    fn zero_cooldown_always_allows() {
        let mut c = Cooldown::new(0, 0);
        c.record_scale_up();
        assert!(c.can_scale_up());
        c.record_scale_down();
        assert!(c.can_scale_down());
    }
}
