/// HTML and JSON report generation for eval runs.
use crate::aggregate::AggregateMetrics;
use crate::breakdown::CaseBreakdown;
use crate::executor::RunId;

/// A full eval run report.
#[derive(Debug, Clone)]
pub struct EvalReport {
    pub run_id: RunId,
    pub suite_name: String,
    pub timestamp: u64,
    pub metrics: AggregateMetrics,
    pub breakdowns: Vec<CaseBreakdown>,
}

impl EvalReport {
    pub fn new(
        run_id: RunId,
        suite_name: String,
        timestamp: u64,
        metrics: AggregateMetrics,
        breakdowns: Vec<CaseBreakdown>,
    ) -> Self {
        Self {
            run_id,
            suite_name,
            timestamp,
            metrics,
            breakdowns,
        }
    }

    /// Generate a JSON representation of the report (no external deps).
    pub fn to_json(&self) -> String {
        let cases_json: Vec<String> = self
            .breakdowns
            .iter()
            .map(|b| {
                format!(
                    r#"{{"case_id":{},"n_rollouts":{},"n_pass":{},"n_fail":{},"pass_rate":{:.4},"mean_latency_ms":{:.2},"total_cost_tokens":{}}}"#,
                    json_str(&b.case_id),
                    b.n_rollouts,
                    b.n_pass,
                    b.n_fail,
                    b.pass_rate,
                    b.mean_latency_ms,
                    b.total_cost_tokens,
                )
            })
            .collect();

        format!(
            r#"{{"run_id":{},"suite_name":{},"timestamp":{},"pass_rate":{:.4},"ci_lower":{:.4},"ci_upper":{:.4},"mean_latency_ms":{:.2},"total_cost_tokens":{},"cases":[{}]}}"#,
            json_str(&self.run_id.0),
            json_str(&self.suite_name),
            self.timestamp,
            self.metrics.pass_rate,
            self.metrics.ci_lower,
            self.metrics.ci_upper,
            self.metrics.mean_latency_ms,
            self.metrics.total_cost_tokens,
            cases_json.join(","),
        )
    }

    /// Generate a minimal HTML report.
    pub fn to_html(&self) -> String {
        let rows: String = self
            .breakdowns
            .iter()
            .map(|b| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.2}%</td><td>{:.1}ms</td></tr>",
                    escape_html(&b.case_id),
                    b.n_pass,
                    b.n_fail,
                    b.pass_rate * 100.0,
                    b.mean_latency_ms,
                )
            })
            .collect();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Eval Report: {suite}</title>
<style>body{{font-family:sans-serif;margin:2rem}}table{{border-collapse:collapse;width:100%}}th,td{{border:1px solid #ccc;padding:0.5rem;text-align:left}}th{{background:#f4f4f4}}</style>
</head>
<body>
<h1>Eval Report: {suite}</h1>
<p>Run ID: <code>{run_id}</code> | Timestamp: {ts}</p>
<p>Pass rate: <strong>{pass_rate:.1}%</strong> 95% CI [{ci_lo:.1}%, {ci_hi:.1}%]</p>
<p>Mean latency: {lat:.1}ms | Total tokens: {tok}</p>
<table>
<thead><tr><th>Case</th><th>Pass</th><th>Fail</th><th>Rate</th><th>Latency</th></tr></thead>
<tbody>{rows}</tbody>
</table>
</body>
</html>"#,
            suite = escape_html(&self.suite_name),
            run_id = escape_html(&self.run_id.0),
            ts = self.timestamp,
            pass_rate = self.metrics.pass_rate * 100.0,
            ci_lo = self.metrics.ci_lower * 100.0,
            ci_hi = self.metrics.ci_upper * 100.0,
            lat = self.metrics.mean_latency_ms,
            tok = self.metrics.total_cost_tokens,
            rows = rows,
        )
    }
}

fn json_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
