/// Plugin manifest format - describes a plugin's identity, version, and capabilities.

/// Semantic version triplet.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse a version string of the form "major.minor.patch".
    pub fn parse(s: &str) -> Result<Self, ManifestError> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(ManifestError::InvalidVersion(s.to_string()));
        }
        let parse = |p: &str| {
            p.parse::<u32>()
                .map_err(|_| ManifestError::InvalidVersion(s.to_string()))
        };
        Ok(Self::new(
            parse(parts[0])?,
            parse(parts[1])?,
            parse(parts[2])?,
        ))
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The type of extension point a plugin implements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginKind {
    Provider,
    VectorStore,
    Tool,
    Memory,
    Guardrail,
    Grader,
    Exporter,
}

impl PluginKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PluginKind::Provider => "provider",
            PluginKind::VectorStore => "vector_store",
            PluginKind::Tool => "tool",
            PluginKind::Memory => "memory",
            PluginKind::Guardrail => "guardrail",
            PluginKind::Grader => "grader",
            PluginKind::Exporter => "exporter",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ManifestError> {
        match s {
            "provider" => Ok(PluginKind::Provider),
            "vector_store" => Ok(PluginKind::VectorStore),
            "tool" => Ok(PluginKind::Tool),
            "memory" => Ok(PluginKind::Memory),
            "guardrail" => Ok(PluginKind::Guardrail),
            "grader" => Ok(PluginKind::Grader),
            "exporter" => Ok(PluginKind::Exporter),
            other => Err(ManifestError::UnknownKind(other.to_string())),
        }
    }
}

/// A plugin manifest describes all metadata required to load and verify a plugin.
#[derive(Debug, Clone, PartialEq)]
pub struct PluginManifest {
    /// Unique plugin identifier (e.g. "acme-openai-provider").
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Plugin version.
    pub version: SemVer,
    /// Minimum SDK API version required.
    pub min_sdk: SemVer,
    /// Maximum SDK API version supported (exclusive upper bound).
    pub max_sdk: SemVer,
    /// The extension kind this plugin implements.
    pub kind: PluginKind,
    /// Optional author string.
    pub author: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Permission scopes this plugin requires.
    pub required_scopes: Vec<String>,
}

/// Errors that can occur while parsing or validating a manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    MissingField(String),
    InvalidVersion(String),
    UnknownKind(String),
    InvalidId(String),
    SdkRangeInverted,
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestError::MissingField(field) => write!(f, "missing required field: {field}"),
            ManifestError::InvalidVersion(v) => write!(f, "invalid version string: {v}"),
            ManifestError::UnknownKind(k) => write!(f, "unknown plugin kind: {k}"),
            ManifestError::InvalidId(id) => write!(f, "invalid plugin id: {id}"),
            ManifestError::SdkRangeInverted => write!(f, "min_sdk must be <= max_sdk"),
        }
    }
}

impl std::error::Error for ManifestError {}

/// Builder for constructing a validated `PluginManifest`.
#[derive(Debug, Default)]
pub struct ManifestBuilder {
    id: Option<String>,
    name: Option<String>,
    version: Option<SemVer>,
    min_sdk: Option<SemVer>,
    max_sdk: Option<SemVer>,
    kind: Option<PluginKind>,
    author: Option<String>,
    description: Option<String>,
    required_scopes: Vec<String>,
}

impl ManifestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn version(mut self, v: SemVer) -> Self {
        self.version = Some(v);
        self
    }

    pub fn sdk_range(mut self, min: SemVer, max: SemVer) -> Self {
        self.min_sdk = Some(min);
        self.max_sdk = Some(max);
        self
    }

    pub fn kind(mut self, kind: PluginKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn scope(mut self, scope: impl Into<String>) -> Self {
        self.required_scopes.push(scope.into());
        self
    }

    pub fn build(self) -> Result<PluginManifest, ManifestError> {
        let id = self
            .id
            .ok_or_else(|| ManifestError::MissingField("id".into()))?;
        if id.is_empty() || id.contains(' ') {
            return Err(ManifestError::InvalidId(id));
        }
        let name = self
            .name
            .ok_or_else(|| ManifestError::MissingField("name".into()))?;
        let version = self
            .version
            .ok_or_else(|| ManifestError::MissingField("version".into()))?;
        let min_sdk = self
            .min_sdk
            .ok_or_else(|| ManifestError::MissingField("min_sdk".into()))?;
        let max_sdk = self
            .max_sdk
            .ok_or_else(|| ManifestError::MissingField("max_sdk".into()))?;
        if min_sdk > max_sdk {
            return Err(ManifestError::SdkRangeInverted);
        }
        let kind = self
            .kind
            .ok_or_else(|| ManifestError::MissingField("kind".into()))?;
        Ok(PluginManifest {
            id,
            name,
            version,
            min_sdk,
            max_sdk,
            kind,
            author: self.author,
            description: self.description,
            required_scopes: self.required_scopes,
        })
    }
}
