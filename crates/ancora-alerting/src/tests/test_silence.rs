#[cfg(test)]
mod tests {
    use crate::silence::{MaintenanceWindow, SilenceRegistry};

    #[test]
    fn maintenance_window_active_during_period() {
        let w = MaintenanceWindow::new("deploy", 1000, 2000, None);
        assert!(w.is_active(1500));
        assert!(!w.is_active(999));
        assert!(!w.is_active(2000));
    }

    #[test]
    fn maintenance_window_silences_all_alerts() {
        let w = MaintenanceWindow::new("deploy", 1000, 2000, None);
        assert!(w.silences("HighErrorRate", 1500));
        assert!(w.silences("WorkerDown", 1500));
    }

    #[test]
    fn maintenance_window_with_filter_silences_only_matching() {
        let w = MaintenanceWindow::new("db-migration", 1000, 2000, Some("WorkerDown".into()));
        assert!(w.silences("WorkerDown", 1500));
        assert!(!w.silences("HighErrorRate", 1500));
    }

    #[test]
    fn silence_registry_is_silenced() {
        let mut reg = SilenceRegistry::default();
        reg.add(MaintenanceWindow::new("maint", 1000, 2000, None));
        assert!(reg.is_silenced("HighErrorRate", 1500));
        assert!(!reg.is_silenced("HighErrorRate", 500));
    }
}
