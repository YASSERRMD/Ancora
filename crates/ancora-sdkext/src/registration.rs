/// Registration helpers for Ancora SDK extensions across all supported languages.
///
/// The registry is the central in-process catalogue of loaded extensions.
/// Language adapters register their wrappers here; the runtime dispatches tool
/// calls by looking up the registry.
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::rs_traits::{ExtensionError, ToolExtension, ToolMeta, Value};

// ---------------------------------------------------------------------------
// Language tag
// ---------------------------------------------------------------------------

/// The source language of a registered extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Go,
    Python,
    TypeScript,
    DotNet,
    Java,
    Other(String),
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Go => write!(f, "go"),
            Language::Python => write!(f, "python"),
            Language::TypeScript => write!(f, "typescript"),
            Language::DotNet => write!(f, "dotnet"),
            Language::Java => write!(f, "java"),
            Language::Other(s) => write!(f, "{s}"),
        }
    }
}

// ---------------------------------------------------------------------------
// Registry entry
// ---------------------------------------------------------------------------

/// An entry in the extension registry.
pub struct RegistryEntry {
    pub language: Language,
    pub meta: ToolMeta,
    pub extension: Arc<dyn ToolExtension>,
}

// ---------------------------------------------------------------------------
// Extension registry
// ---------------------------------------------------------------------------

/// Thread-safe registry of all loaded extensions, keyed by extension name.
pub struct ExtensionRegistry {
    entries: RwLock<HashMap<String, RegistryEntry>>,
}

impl ExtensionRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        ExtensionRegistry {
            entries: RwLock::new(HashMap::new()),
        }
    }

    /// Register an extension.  Returns an error if the name is already taken.
    pub fn register(
        &self,
        language: Language,
        extension: Arc<dyn ToolExtension>,
    ) -> Result<(), ExtensionError> {
        let meta = extension.meta();
        let name = meta.name.clone();
        let mut guard = self
            .entries
            .write()
            .map_err(|_| ExtensionError::ExecutionFailed("registry lock poisoned".to_string()))?;
        if guard.contains_key(&name) {
            return Err(ExtensionError::InvalidArgument(format!(
                "extension '{name}' is already registered"
            )));
        }
        guard.insert(
            name,
            RegistryEntry {
                language,
                meta,
                extension,
            },
        );
        Ok(())
    }

    /// Look up an extension by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn ToolExtension>> {
        self.entries
            .read()
            .ok()?
            .get(name)
            .map(|e| Arc::clone(&e.extension))
    }

    /// Return metadata for every registered extension.
    pub fn list(&self) -> Vec<ToolMeta> {
        self.entries
            .read()
            .map(|g| g.values().map(|e| e.meta.clone()).collect())
            .unwrap_or_default()
    }

    /// Return the number of registered extensions.
    pub fn len(&self) -> usize {
        self.entries.read().map(|g| g.len()).unwrap_or(0)
    }

    /// Return true if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Dispatch a tool call by name.
    pub fn dispatch(
        &self,
        name: &str,
        args: HashMap<String, Value>,
    ) -> Result<Value, ExtensionError> {
        let ext = self
            .get(name)
            .ok_or_else(|| ExtensionError::NotSupported(format!("extension '{name}' not found")))?;
        ext.execute(args)
    }

    /// Remove all entries (useful for test isolation).
    pub fn clear(&self) {
        if let Ok(mut g) = self.entries.write() {
            g.clear();
        }
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Per-language registration helpers
// ---------------------------------------------------------------------------

/// Register a Rust extension with the registry.
pub fn register_rust_extension(
    registry: &ExtensionRegistry,
    extension: Arc<dyn ToolExtension>,
) -> Result<(), ExtensionError> {
    registry.register(Language::Rust, extension)
}

/// Register a Go extension with the registry.
pub fn register_go_extension(
    registry: &ExtensionRegistry,
    extension: Arc<dyn ToolExtension>,
) -> Result<(), ExtensionError> {
    registry.register(Language::Go, extension)
}

/// Register a Python extension with the registry.
pub fn register_python_extension(
    registry: &ExtensionRegistry,
    extension: Arc<dyn ToolExtension>,
) -> Result<(), ExtensionError> {
    registry.register(Language::Python, extension)
}

/// Register a TypeScript extension with the registry.
pub fn register_typescript_extension(
    registry: &ExtensionRegistry,
    extension: Arc<dyn ToolExtension>,
) -> Result<(), ExtensionError> {
    registry.register(Language::TypeScript, extension)
}

/// Register a .NET extension with the registry.
pub fn register_dotnet_extension(
    registry: &ExtensionRegistry,
    extension: Arc<dyn ToolExtension>,
) -> Result<(), ExtensionError> {
    registry.register(Language::DotNet, extension)
}

/// Register a Java extension with the registry.
pub fn register_java_extension(
    registry: &ExtensionRegistry,
    extension: Arc<dyn ToolExtension>,
) -> Result<(), ExtensionError> {
    registry.register(Language::Java, extension)
}
