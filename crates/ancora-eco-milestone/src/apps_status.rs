/// Sample application run status for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppRunResult {
    Ok,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct AppStatus {
    pub app_name: String,
    pub version: String,
    pub result: AppRunResult,
}

impl AppStatus {
    pub fn ok(app_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
            version: version.into(),
            result: AppRunResult::Ok,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.result == AppRunResult::Ok
    }
}

pub fn sample_app_statuses() -> Vec<AppStatus> {
    vec![
        AppStatus::ok("single-agent", "0.1.0"),
        AppStatus::ok("multi-agent-verifier", "0.1.0"),
        AppStatus::ok("streaming-chat", "0.1.0"),
        AppStatus::ok("structured-output", "0.1.0"),
        AppStatus::ok("mcp-tool", "0.1.0"),
        AppStatus::ok("human-in-loop", "0.1.0"),
        AppStatus::ok("rag-lancedb", "0.1.0"),
        AppStatus::ok("sqlite-persistence", "0.1.0"),
        AppStatus::ok("cost-otel", "0.1.0"),
        AppStatus::ok("advanced-parity", "0.1.0"),
    ]
}

pub fn all_apps_ok(statuses: &[AppStatus]) -> bool {
    statuses.iter().all(|s| s.is_ok())
}
