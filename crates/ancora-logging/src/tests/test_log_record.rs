#[cfg(test)]
mod tests {
    use crate::log_record::{LogLevel, LogRecord};

    #[test]
    fn logs_are_valid_json_with_ids() {
        let rec = LogRecord::new(LogLevel::Info, "ancora-worker", "run started", 1000)
            .with_correlation("run-1", "tenant-a", "trace-xyz");
        let json = rec.to_json();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["run_id"], "run-1");
        assert_eq!(v["tenant_id"], "tenant-a");
        assert_eq!(v["trace_id"], "trace-xyz");
    }

    #[test]
    fn log_record_level_serializes() {
        let rec = LogRecord::new(LogLevel::Warn, "m", "msg", 0);
        let json = rec.to_json();
        assert!(json.contains("Warn"));
    }

    #[test]
    fn log_record_roundtrip() {
        let rec = LogRecord::new(LogLevel::Error, "core", "fail", 42);
        let json = rec.to_json();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["timestamp_secs"], 42);
    }
}
