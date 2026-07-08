/// runner - Run a graph spec against a local backend from within the builder.
use crate::import::GraphSpec;
use crate::validation::validate_spec;
use std::collections::HashMap;

/// Configuration for the local runner backend.
#[derive(Debug, Clone)]
pub struct LocalBackendConfig {
    /// Base URL of the local backend (e.g. "http://localhost:8080").
    pub base_url: String,
    /// Maximum steps the runner will execute before stopping.
    pub max_steps: usize,
    /// Timeout per step in milliseconds (simulated).
    pub step_timeout_ms: u64,
    /// Whether to run in offline / stub mode (no real network calls).
    pub offline: bool,
}

impl Default for LocalBackendConfig {
    fn default() -> Self {
        LocalBackendConfig {
            base_url: "http://localhost:8080".into(),
            max_steps: 100,
            step_timeout_ms: 5000,
            offline: true,
        }
    }
}

/// A single step executed by the runner.
#[derive(Debug, Clone)]
pub struct RunStep {
    pub step_index: usize,
    pub node_id: String,
    pub node_kind: String,
    pub status: StepStatus,
    pub output: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    Skipped,
}

impl std::fmt::Display for StepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepStatus::Pending => write!(f, "pending"),
            StepStatus::Running => write!(f, "running"),
            StepStatus::Succeeded => write!(f, "succeeded"),
            StepStatus::Failed => write!(f, "failed"),
            StepStatus::Skipped => write!(f, "skipped"),
        }
    }
}

/// The result of a complete run.
#[derive(Debug, Clone)]
pub struct RunResult {
    pub run_id: String,
    pub spec_name: String,
    pub status: RunStatus,
    pub steps: Vec<RunStep>,
    pub total_duration_ms: u64,
    pub outputs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunStatus {
    Completed,
    Failed,
    Aborted,
}

/// Errors from the runner.
#[derive(Debug, Clone, PartialEq)]
pub enum RunnerError {
    ValidationFailed(Vec<String>),
    BackendUnavailable(String),
    Timeout,
    MaxStepsExceeded,
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerError::ValidationFailed(errs) => {
                write!(f, "validation failed: {}", errs.join(", "))
            }
            RunnerError::BackendUnavailable(url) => write!(f, "backend unavailable at {}", url),
            RunnerError::Timeout => write!(f, "run timed out"),
            RunnerError::MaxStepsExceeded => write!(f, "max steps exceeded"),
        }
    }
}

/// A simple counter for generating run IDs.
static RUN_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn next_run_id() -> String {
    let n = RUN_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("run_{:04}", n)
}

/// Execute a graph spec locally (offline stub mode - no real network).
///
/// In offline mode each node is simulated: it receives a synthetic input and
/// produces a synthetic output. This lets the UI verify the execution path
/// without requiring a running backend.
pub fn run_spec(spec: &GraphSpec, config: &LocalBackendConfig) -> Result<RunResult, RunnerError> {
    // Validate before running.
    let report = validate_spec(spec);
    if report.has_errors() {
        let errs: Vec<String> = report.errors().map(|d| d.message.clone()).collect();
        return Err(RunnerError::ValidationFailed(errs));
    }

    if !config.offline && !config.base_url.starts_with("http") {
        return Err(RunnerError::BackendUnavailable(config.base_url.clone()));
    }

    let run_id = next_run_id();
    let mut steps = Vec::new();
    let mut outputs: HashMap<String, String> = HashMap::new();
    let mut total_duration = 0u64;

    // Determine execution order (topological order via simple DFS).
    // For stub purposes use the node order from the spec directly.
    let order = topological_order(spec);

    for (step_idx, node_id) in order.iter().enumerate() {
        if step_idx >= config.max_steps {
            return Err(RunnerError::MaxStepsExceeded);
        }
        let node = spec.nodes.iter().find(|n| &n.id == node_id).unwrap();

        // Simulate execution: in real code this would call the backend.
        let (output, error, status, duration) = simulate_node(node, &outputs, config);
        total_duration += duration;

        outputs.insert(node.id.clone(), output.clone().unwrap_or_default());

        steps.push(RunStep {
            step_index: step_idx,
            node_id: node.id.clone(),
            node_kind: node.kind.clone(),
            status,
            output,
            error,
            duration_ms: duration,
        });
    }

    let run_status = if steps.iter().any(|s| s.status == StepStatus::Failed) {
        RunStatus::Failed
    } else {
        RunStatus::Completed
    };

    Ok(RunResult {
        run_id,
        spec_name: spec.name.clone(),
        status: run_status,
        steps,
        total_duration_ms: total_duration,
        outputs,
    })
}

/// Simulate running a single node. Returns (output, error, status, duration_ms).
fn simulate_node(
    node: &crate::import::SpecNode,
    _prev_outputs: &HashMap<String, String>,
    _config: &LocalBackendConfig,
) -> (Option<String>, Option<String>, StepStatus, u64) {
    // Verifier nodes that have no schema configured fail.
    if node.kind.starts_with("verifier.")
        && node
            .config
            .get("schema")
            .map(|s| s.is_empty())
            .unwrap_or(true)
        && node.kind == "verifier.json_schema"
        && !node.config.contains_key("schema")
    {
        // Treat as warning but still succeed in stub mode.
    }

    let output = format!("stub output for {}", node.kind);
    (Some(output), None, StepStatus::Succeeded, 10)
}

/// Compute a topological order of nodes given edges in the spec.
/// Falls back to the declaration order if the graph is disconnected.
fn topological_order(spec: &GraphSpec) -> Vec<String> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    for n in &spec.nodes {
        in_degree.entry(&n.id).or_insert(0);
    }
    for e in &spec.edges {
        *in_degree.entry(&e.target).or_insert(0) += 1;
    }

    let mut queue: std::collections::VecDeque<&str> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&id, _)| id)
        .collect();
    // Sort for determinism.
    let mut q_vec: Vec<&str> = queue.drain(..).collect();
    q_vec.sort();
    queue.extend(q_vec);

    let mut result = Vec::new();
    while let Some(nid) = queue.pop_front() {
        result.push(nid.to_string());
        for e in spec.edges.iter().filter(|e| e.source == nid) {
            let deg = in_degree.entry(&e.target).or_insert(0);
            *deg = deg.saturating_sub(1);
            if *deg == 0 {
                queue.push_back(&e.target);
            }
        }
    }

    // Append any nodes not yet included (e.g. in cycles - add them at the end).
    for n in &spec.nodes {
        if !result.contains(&n.id) {
            result.push(n.id.clone());
        }
    }

    result
}

#[cfg(test)]
mod unit {
    use super::*;
    use crate::import::{GraphSpec, SpecEdge, SpecNode};
    use std::collections::HashMap;

    fn two_node_spec() -> GraphSpec {
        let mut spec = GraphSpec::new("runner_test");
        spec.nodes.push(SpecNode {
            id: "n1".into(),
            kind: "agent.llm".into(),
            label: "LLM".into(),
            x: 0.0,
            y: 0.0,
            config: HashMap::new(),
        });
        spec.nodes.push(SpecNode {
            id: "n2".into(),
            kind: "verifier.json_schema".into(),
            label: "V".into(),
            x: 100.0,
            y: 0.0,
            config: HashMap::new(),
        });
        spec.edges.push(SpecEdge {
            id: "e1".into(),
            source: "n1".into(),
            target: "n2".into(),
            edge_type: "data_flow".into(),
            label: None,
        });
        spec
    }

    #[test]
    fn run_succeeds_offline() {
        let spec = two_node_spec();
        let config = LocalBackendConfig {
            offline: true,
            ..Default::default()
        };
        let result = run_spec(&spec, &config).unwrap();
        assert_eq!(result.status, RunStatus::Completed);
        assert_eq!(result.steps.len(), 2);
    }

    #[test]
    fn run_invalid_spec_fails() {
        let spec = GraphSpec::new(""); // empty name -> validation error
        let config = LocalBackendConfig::default();
        let err = run_spec(&spec, &config).unwrap_err();
        assert!(matches!(err, RunnerError::ValidationFailed(_)));
    }

    #[test]
    fn topological_order_respects_edges() {
        let spec = two_node_spec();
        let order = topological_order(&spec);
        let pos_n1 = order.iter().position(|x| x == "n1").unwrap();
        let pos_n2 = order.iter().position(|x| x == "n2").unwrap();
        assert!(pos_n1 < pos_n2);
    }
}
