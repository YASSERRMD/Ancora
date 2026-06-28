/// Local-first model registry.
///
/// Each sample app resolves its model through this registry so that
/// no remote endpoint is ever contacted during offline or air-gapped runs.

#[derive(Debug, Clone, PartialEq)]
pub enum ModelBackend {
    /// GGUF or GGML model loaded from a local file path.
    LocalFile { path: String },
    /// In-process stub used for unit testing.
    Stub { name: String },
}

#[derive(Debug, Clone)]
pub struct ModelDescriptor {
    pub id: String,
    pub backend: ModelBackend,
    pub context_tokens: usize,
}

impl ModelDescriptor {
    pub fn new(
        id: impl Into<String>,
        backend: ModelBackend,
        context_tokens: usize,
    ) -> Self {
        Self {
            id: id.into(),
            backend,
            context_tokens,
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(
            self.backend,
            ModelBackend::LocalFile { .. } | ModelBackend::Stub { .. }
        )
    }
}

#[derive(Debug, Default)]
pub struct ModelRegistry {
    models: Vec<ModelDescriptor>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, descriptor: ModelDescriptor) {
        self.models.push(descriptor);
    }

    pub fn get(&self, id: &str) -> Option<&ModelDescriptor> {
        self.models.iter().find(|m| m.id == id)
    }

    /// Return all models backed by local resources (offline-safe).
    pub fn local_models(&self) -> Vec<&ModelDescriptor> {
        self.models.iter().filter(|m| m.is_local()).collect()
    }

    pub fn count(&self) -> usize {
        self.models.len()
    }
}

/// Simplified inference call result - fully local, no network.
#[derive(Debug, Clone)]
pub struct InferenceResult {
    pub model_id: String,
    pub output: String,
    pub tokens_used: usize,
}

/// Run a stub inference against a local model descriptor.
pub fn run_local_inference(
    model: &ModelDescriptor,
    prompt: &str,
) -> Result<InferenceResult, String> {
    if !model.is_local() {
        return Err(format!(
            "model '{}' is not local; cannot run offline",
            model.id
        ));
    }
    // Stub: echo the prompt back with a prefix.
    let output = format!("[{}] {}", model.id, prompt);
    Ok(InferenceResult {
        model_id: model.id.clone(),
        output,
        tokens_used: prompt.split_whitespace().count(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_registered_models_are_local() {
        let mut registry = ModelRegistry::new();
        registry.register(ModelDescriptor::new(
            "qa-model",
            ModelBackend::Stub { name: "qa-stub".to_string() },
            4096,
        ));
        registry.register(ModelDescriptor::new(
            "code-model",
            ModelBackend::LocalFile { path: "/models/code.gguf".to_string() },
            8192,
        ));
        assert_eq!(registry.local_models().len(), registry.count());
    }
}
