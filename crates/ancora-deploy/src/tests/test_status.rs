#[cfg(test)]
mod tests {
    use crate::{DeployStatus, Version};

    #[test]
    fn deploy_status_tracks_switches() {
        let mut s = DeployStatus::new();
        assert_eq!(s.total_switches, 0);
        s.record_switch(Version::new(2, 0, 0));
        assert_eq!(s.total_switches, 1);
        assert_eq!(s.active_version.as_ref().unwrap().major, 2);
    }

    #[test]
    fn canary_status_reflects_active_pct() {
        let mut s = DeployStatus::new();
        s.set_canary(0.1);
        assert!(s.canary_active);
        s.set_canary(0.0);
        assert!(!s.canary_active);
    }
}
