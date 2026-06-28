/// Parity end-to-end: extension parity checks across language runtimes.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageRuntime {
    Rust,
    Python,
    JavaScript,
    Go,
    Wasm,
}

impl LanguageRuntime {
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageRuntime::Rust => "rust",
            LanguageRuntime::Python => "python",
            LanguageRuntime::JavaScript => "javascript",
            LanguageRuntime::Go => "go",
            LanguageRuntime::Wasm => "wasm",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtensionCapabilities {
    pub runtime: LanguageRuntime,
    pub supports_streaming: bool,
    pub supports_tool_calls: bool,
    pub supports_memory: bool,
    pub max_payload_bytes: usize,
}

impl ExtensionCapabilities {
    pub fn new(runtime: LanguageRuntime) -> Self {
        ExtensionCapabilities {
            runtime,
            supports_streaming: true,
            supports_tool_calls: true,
            supports_memory: true,
            max_payload_bytes: 1024 * 1024,
        }
    }

    pub fn parity_score(&self, other: &ExtensionCapabilities) -> f64 {
        let mut matches = 0u32;
        let total = 3u32;
        if self.supports_streaming == other.supports_streaming {
            matches += 1;
        }
        if self.supports_tool_calls == other.supports_tool_calls {
            matches += 1;
        }
        if self.supports_memory == other.supports_memory {
            matches += 1;
        }
        matches as f64 / total as f64
    }

    pub fn is_parity_with(&self, other: &ExtensionCapabilities) -> bool {
        self.parity_score(other) >= 1.0
    }
}

pub fn check_cross_runtime_parity(caps: &[ExtensionCapabilities]) -> Vec<(String, String, f64)> {
    let mut results = Vec::new();
    for i in 0..caps.len() {
        for j in (i + 1)..caps.len() {
            let score = caps[i].parity_score(&caps[j]);
            results.push((
                caps[i].runtime.as_str().to_string(),
                caps[j].runtime.as_str().to_string(),
                score,
            ));
        }
    }
    results
}
