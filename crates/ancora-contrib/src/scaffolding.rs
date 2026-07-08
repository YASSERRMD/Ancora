//! ancora-contrib: template scaffolding command
//!
//! Provides a programmatic API for generating contribution scaffolds
//! (directory layout, source stubs, test stubs, docs stub).
//!
//! In a real CLI this would write files to disk; here the logic produces
//! `ScaffoldOutput` values that can be inspected in tests without touching
//! the filesystem.

/// The kind of extension point to scaffold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldKind {
    Provider,
    VectorStore,
    Tool,
    Grader,
    Guardrail,
    Exporter,
    Plugin,
}

impl ScaffoldKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScaffoldKind::Provider => "provider",
            ScaffoldKind::VectorStore => "vectorstore",
            ScaffoldKind::Tool => "tool",
            ScaffoldKind::Grader => "grader",
            ScaffoldKind::Guardrail => "guardrail",
            ScaffoldKind::Exporter => "exporter",
            ScaffoldKind::Plugin => "plugin",
        }
    }
}

impl std::str::FromStr for ScaffoldKind {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "provider" => Ok(ScaffoldKind::Provider),
            "vectorstore" | "vector_store" | "vector-store" => Ok(ScaffoldKind::VectorStore),
            "tool" => Ok(ScaffoldKind::Tool),
            "grader" => Ok(ScaffoldKind::Grader),
            "guardrail" => Ok(ScaffoldKind::Guardrail),
            "exporter" => Ok(ScaffoldKind::Exporter),
            "plugin" => Ok(ScaffoldKind::Plugin),
            other => Err(ScaffoldError::UnknownKind(other.to_string())),
        }
    }
}

/// Request to scaffold a new contribution.
#[derive(Debug, Clone)]
pub struct ScaffoldRequest {
    /// The extension-point kind to scaffold.
    pub kind: ScaffoldKind,
    /// Human-readable name for the new contribution (e.g. "AcmeCloud").
    pub name: String,
    /// Optional author string.
    pub author: Option<String>,
}

impl ScaffoldRequest {
    pub fn new(kind: ScaffoldKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            author: None,
        }
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Derive a snake_case identifier from `name`.
    pub fn snake_name(&self) -> String {
        self.name
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    vec!['_', c.to_ascii_lowercase()]
                } else {
                    vec![c.to_ascii_lowercase()]
                }
            })
            .collect()
    }
}

/// A generated file (path relative to the crate root, content as a string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

/// The complete set of files produced by a scaffold operation.
#[derive(Debug, Clone)]
pub struct ScaffoldOutput {
    pub files: Vec<GeneratedFile>,
}

impl ScaffoldOutput {
    pub fn get(&self, path: &str) -> Option<&GeneratedFile> {
        self.files.iter().find(|f| f.path == path)
    }

    pub fn paths(&self) -> Vec<&str> {
        self.files.iter().map(|f| f.path.as_str()).collect()
    }
}

/// Errors from the scaffolding command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaffoldError {
    UnknownKind(String),
    InvalidName(String),
}

impl std::fmt::Display for ScaffoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaffoldError::UnknownKind(k) => write!(f, "unknown scaffold kind: {k}"),
            ScaffoldError::InvalidName(n) => write!(f, "invalid scaffold name: {n}"),
        }
    }
}

impl std::error::Error for ScaffoldError {}

/// Validate and run a scaffold request.
pub fn scaffold(req: &ScaffoldRequest) -> Result<ScaffoldOutput, ScaffoldError> {
    if req.name.is_empty() || req.name.contains(' ') {
        return Err(ScaffoldError::InvalidName(req.name.clone()));
    }

    let kind_str = req.kind.as_str();
    let snake = req.snake_name();
    let author = req.author.as_deref().unwrap_or("YASSERRMD");

    let src_lib = format!(
        "/// {name} - a {kind} plugin for ancora.\n\nmod {snake};\n\n#[cfg(test)]\nmod tests;\n",
        name = req.name,
        kind = kind_str,
        snake = snake
    );

    let src_impl = format!(
        "/// {name} {kind} implementation.\n\n// TODO: implement {snake} here.\n",
        name = req.name,
        kind = kind_str,
        snake = snake
    );

    let src_test = format!(
        "#[cfg(test)]\nmod tests {{\n    use super::super::{snake}::*;\n\n    #[test]\n    fn {snake}_placeholder_test() {{\n        // TODO: write meaningful tests.\n        assert!(true);\n    }}\n}}\n",
        snake = snake
    );

    let cargo_toml = format!(
        "[package]\nname = \"ancora-{snake}\"\nversion = \"0.1.0\"\nedition = \"2021\"\nauthors = [\"{author} <{author}@example.com>\"]\nlicense = \"Apache-2.0\"\n\n[dependencies]\n",
        snake = snake,
        author = author
    );

    let docs_stub = format!(
        "# {name}\n\nThis is a {kind} plugin for ancora.\n\n## Usage\n\nTODO: describe how to use this plugin.\n\n## Contributing\n\nSee [CONTRIBUTING.md](../../CONTRIBUTING.md).\n",
        name = req.name,
        kind = kind_str
    );

    let conformance_stub = format!(
        "/// Conformance harness stub for {name}.\n\n#[cfg(test)]\nmod conformance {{\n    #[test]\n    fn {snake}_conforms() {{\n        // TODO: run conformance checks.\n        assert!(true);\n    }}\n}}\n",
        name = req.name,
        snake = snake
    );

    Ok(ScaffoldOutput {
        files: vec![
            GeneratedFile {
                path: "Cargo.toml".to_string(),
                content: cargo_toml,
            },
            GeneratedFile {
                path: "src/lib.rs".to_string(),
                content: src_lib,
            },
            GeneratedFile {
                path: format!("src/{snake}.rs"),
                content: src_impl,
            },
            GeneratedFile {
                path: "src/tests.rs".to_string(),
                content: src_test,
            },
            GeneratedFile {
                path: "docs/README.md".to_string(),
                content: docs_stub,
            },
            GeneratedFile {
                path: "src/conformance.rs".to_string(),
                content: conformance_stub,
            },
        ],
    })
}
