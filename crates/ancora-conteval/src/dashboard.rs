/// Continuous evaluation dashboard JSON serialization.
///
/// Produces a structured JSON-compatible representation of the current
/// evaluation state for consumption by monitoring UIs and external tooling.
/// Implemented without external dependencies using manual JSON building.
use crate::alerting::{AlertSeverity, QualityAlert};

/// A snapshot of a model's quality metrics for the dashboard.
#[derive(Debug, Clone)]
pub struct ModelSnapshot {
    pub model: String,
    pub provider: String,
    pub mean_score: f64,
    pub latest_score: f64,
    pub observation_count: usize,
    pub trend_slope: Option<f64>,
}

/// A snapshot of a provider's aggregated metrics.
#[derive(Debug, Clone)]
pub struct ProviderSnapshot {
    pub provider: String,
    pub mean_score: f64,
    pub observation_count: usize,
    pub model_count: usize,
}

/// Summary of active alerts.
#[derive(Debug, Clone)]
pub struct AlertSummary {
    pub total: usize,
    pub critical: usize,
    pub warning: usize,
    pub info: usize,
}

impl AlertSummary {
    pub fn from_alerts(alerts: &[QualityAlert]) -> Self {
        let critical = alerts
            .iter()
            .filter(|a| a.severity == AlertSeverity::Critical)
            .count();
        let warning = alerts
            .iter()
            .filter(|a| a.severity == AlertSeverity::Warning)
            .count();
        let info = alerts
            .iter()
            .filter(|a| a.severity == AlertSeverity::Info)
            .count();
        AlertSummary {
            total: alerts.len(),
            critical,
            warning,
            info,
        }
    }
}

/// Full dashboard state.
#[derive(Debug, Clone)]
pub struct DashboardState {
    pub generated_at: u64,
    pub models: Vec<ModelSnapshot>,
    pub providers: Vec<ProviderSnapshot>,
    pub alert_summary: AlertSummary,
}

impl DashboardState {
    pub fn new(
        generated_at: u64,
        models: Vec<ModelSnapshot>,
        providers: Vec<ProviderSnapshot>,
        alert_summary: AlertSummary,
    ) -> Self {
        DashboardState {
            generated_at,
            models,
            providers,
            alert_summary,
        }
    }

    /// Serialize this state to a JSON string without external dependencies.
    pub fn to_json(&self) -> String {
        let models_json: Vec<String> = self
            .models
            .iter()
            .map(|m| {
                let trend = match m.trend_slope {
                    Some(s) => format!("{:.6}", s),
                    None => "null".to_string(),
                };
                format!(
                    r#"{{"model":"{model}","provider":"{provider}","mean_score":{mean:.6},"latest_score":{latest:.6},"observation_count":{obs},"trend_slope":{trend}}}"#,
                    model = escape_json(&m.model),
                    provider = escape_json(&m.provider),
                    mean = m.mean_score,
                    latest = m.latest_score,
                    obs = m.observation_count,
                    trend = trend,
                )
            })
            .collect();

        let providers_json: Vec<String> = self
            .providers
            .iter()
            .map(|p| {
                format!(
                    r#"{{"provider":"{provider}","mean_score":{mean:.6},"observation_count":{obs},"model_count":{mc}}}"#,
                    provider = escape_json(&p.provider),
                    mean = p.mean_score,
                    obs = p.observation_count,
                    mc = p.model_count,
                )
            })
            .collect();

        format!(
            r#"{{"generated_at":{ts},"models":[{models}],"providers":[{providers}],"alert_summary":{{"total":{total},"critical":{crit},"warning":{warn},"info":{info}}}}}"#,
            ts = self.generated_at,
            models = models_json.join(","),
            providers = providers_json.join(","),
            total = self.alert_summary.total,
            crit = self.alert_summary.critical,
            warn = self.alert_summary.warning,
            info = self.alert_summary.info,
        )
    }
}

/// Escape a string for embedding in JSON (handles backslash and double quotes).
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Validate that a string is well-formed JSON by checking basic structure.
/// Returns `Ok(())` for valid JSON-like strings, `Err` with a description
/// for obvious issues.
pub fn validate_json(json: &str) -> Result<(), String> {
    let trimmed = json.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("JSON must be a top-level object".to_string());
    }
    // Check balanced braces and brackets.
    let mut depth_brace = 0i32;
    let mut depth_bracket = 0i32;
    let mut in_string = false;
    let mut prev_char = '\0';
    for c in trimmed.chars() {
        if in_string {
            if c == '"' && prev_char != '\\' {
                in_string = false;
            }
        } else {
            match c {
                '"' => in_string = true,
                '{' => depth_brace += 1,
                '}' => depth_brace -= 1,
                '[' => depth_bracket += 1,
                ']' => depth_bracket -= 1,
                _ => {}
            }
        }
        prev_char = c;
    }
    if depth_brace != 0 {
        return Err(format!("unbalanced braces: depth={}", depth_brace));
    }
    if depth_bracket != 0 {
        return Err(format!("unbalanced brackets: depth={}", depth_bracket));
    }
    Ok(())
}
