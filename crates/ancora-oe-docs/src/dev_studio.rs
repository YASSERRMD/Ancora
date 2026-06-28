//! Developer studio helpers: local eval runs, trace replay, and prompt playgrounds.

/// Configuration for a local dev studio session.
#[derive(Debug, Clone)]
pub struct DevStudioConfig {
    pub port: u16,
    pub enable_trace_replay: bool,
    pub enable_prompt_playground: bool,
    pub enable_local_evals: bool,
}

impl Default for DevStudioConfig {
    fn default() -> Self {
        Self {
            port: 7878,
            enable_trace_replay: true,
            enable_prompt_playground: true,
            enable_local_evals: true,
        }
    }
}

/// A trace replay request loaded from a stored trace file.
#[derive(Debug, Clone)]
pub struct TraceReplayRequest {
    pub trace_id: String,
    pub replay_as_of_ms: Option<u64>,
}

impl TraceReplayRequest {
    pub fn new(trace_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            replay_as_of_ms: None,
        }
    }

    pub fn at_time(mut self, ms: u64) -> Self {
        self.replay_as_of_ms = Some(ms);
        self
    }
}

/// A prompt variation to test in the playground.
#[derive(Debug, Clone)]
pub struct PromptVariant {
    pub id: String,
    pub template: String,
    pub variables: Vec<String>,
}

impl PromptVariant {
    pub fn new(id: impl Into<String>, template: impl Into<String>) -> Self {
        let template = template.into();
        // Extract {variable} placeholders.
        let variables = extract_variables(&template);
        Self {
            id: id.into(),
            template,
            variables,
        }
    }
}

fn extract_variables(template: &str) -> Vec<String> {
    let mut vars = Vec::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' {
            let mut var = String::new();
            for inner in chars.by_ref() {
                if inner == '}' {
                    break;
                }
                var.push(inner);
            }
            if !var.is_empty() {
                vars.push(var);
            }
        }
    }
    vars
}
