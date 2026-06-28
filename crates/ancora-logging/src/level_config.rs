use std::collections::HashMap;
use crate::log_record::LogLevel;

/// Per-module log level configuration.
#[derive(Default)]
pub struct LevelConfig {
    global: LogLevel,
    per_module: HashMap<String, LogLevel>,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl LevelConfig {
    pub fn new(global: LogLevel) -> Self {
        Self { global, per_module: HashMap::new() }
    }

    pub fn set_module(&mut self, module: impl Into<String>, level: LogLevel) {
        self.per_module.insert(module.into(), level);
    }

    pub fn is_enabled(&self, module: &str, level: &LogLevel) -> bool {
        let threshold = self.per_module.get(module).unwrap_or(&self.global);
        level.numeric() >= threshold.numeric()
    }
}
