use crate::versioning::Version;

/// A single entry to be published to the registry.
#[derive(Debug, Clone)]
pub struct PublishEntry {
    /// Entry name (e.g., "my-tool").
    pub name: String,
    /// Semantic version of this entry.
    pub version: Version,
    /// Raw payload bytes (serialized form of the entry).
    pub payload: Vec<u8>,
    /// Publisher identity (token, username, key fingerprint, etc.).
    pub publisher: String,
    /// Optional detached signature over the payload.
    pub signature: Option<String>,
}

impl PublishEntry {
    /// Construct a minimal unsigned publish entry.
    pub fn new(
        name: impl Into<String>,
        version: Version,
        payload: Vec<u8>,
        publisher: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version,
            payload,
            publisher: publisher.into(),
            signature: None,
        }
    }

    /// Attach a signature to this entry.
    pub fn with_signature(mut self, sig: impl Into<String>) -> Self {
        self.signature = Some(sig.into());
        self
    }
}

/// Errors that can occur during a publish operation.
#[derive(Debug, PartialEq, Eq)]
pub enum PublishError {
    /// The publisher is not authorised to publish to this registry.
    AccessDenied(String),
    /// Strict mode is active but no signature was provided.
    MissingSignature,
    /// The provided signature does not match the stored trusted key.
    InvalidSignature,
    /// The entry already exists at this version and the registry does not allow overwrites.
    AlreadyExists,
    /// Generic validation failure with a descriptive message.
    ValidationError(String),
}

impl std::fmt::Display for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessDenied(r) => write!(f, "access denied: {r}"),
            Self::MissingSignature => write!(f, "signature required in strict mode"),
            Self::InvalidSignature => write!(f, "signature verification failed"),
            Self::AlreadyExists => write!(f, "entry already exists at this version"),
            Self::ValidationError(m) => write!(f, "validation error: {m}"),
        }
    }
}
