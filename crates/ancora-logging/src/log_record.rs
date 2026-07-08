use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn numeric(&self) -> u8 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }
}

/// A structured JSON log record.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogRecord {
    pub timestamp_secs: u64,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    /// Correlation identifiers.
    pub run_id: Option<String>,
    pub tenant_id: Option<String>,
    pub trace_id: Option<String>,
    /// Additional structured fields (must not contain secrets).
    pub fields: Value,
}

impl LogRecord {
    pub fn new(
        level: LogLevel,
        module: impl Into<String>,
        message: impl Into<String>,
        timestamp_secs: u64,
    ) -> Self {
        Self {
            timestamp_secs,
            level,
            module: module.into(),
            message: message.into(),
            run_id: None,
            tenant_id: None,
            trace_id: None,
            fields: Value::Object(Default::default()),
        }
    }

    pub fn with_correlation(
        mut self,
        run_id: impl Into<String>,
        tenant_id: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> Self {
        self.run_id = Some(run_id.into());
        self.tenant_id = Some(tenant_id.into());
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
