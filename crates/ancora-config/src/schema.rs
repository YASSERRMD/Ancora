use serde::{Deserialize, Serialize};

/// Top-level configuration schema for Ancora.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AncoraCfg {
    pub core: CoreCfg,
    pub journal: JournalCfg,
    pub worker: WorkerCfg,
    pub telemetry: TelemetryCfg,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreCfg {
    pub log_level: String,
    pub data_dir: String,
    pub max_concurrent_runs: u32,
}

impl Default for CoreCfg {
    fn default() -> Self {
        Self {
            log_level: "info".into(),
            data_dir: "/var/ancora".into(),
            max_concurrent_runs: 8,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JournalCfg {
    pub flush_interval_ms: u64,
    pub max_entries_per_batch: usize,
}

impl Default for JournalCfg {
    fn default() -> Self {
        Self {
            flush_interval_ms: 500,
            max_entries_per_batch: 256,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkerCfg {
    pub concurrency: u32,
    pub heartbeat_ms: u64,
    pub provider: String,
    /// Secret reference key for the provider API key (never store inline).
    pub api_key_ref: Option<String>,
}

impl Default for WorkerCfg {
    fn default() -> Self {
        Self {
            concurrency: 4,
            heartbeat_ms: 5000,
            provider: "openai".into(),
            api_key_ref: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelemetryCfg {
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    pub scrape_interval_ms: u64,
}

impl Default for TelemetryCfg {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            tracing_enabled: false,
            scrape_interval_ms: 15000,
        }
    }
}
