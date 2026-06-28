use crate::versioning::Version;

/// The outcome of a fetch request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchResult {
    /// Entry found; contains the payload bytes.
    Found(Vec<u8>),
    /// No entry exists for the requested (name, version) pair.
    NotFound,
}

impl FetchResult {
    /// Returns true if this result contains a payload.
    pub fn is_found(&self) -> bool {
        matches!(self, Self::Found(_))
    }

    /// Unwrap the payload, panicking only in test/debug contexts.
    pub fn unwrap_payload(self) -> Vec<u8> {
        match self {
            Self::Found(p) => p,
            Self::NotFound => panic!("FetchResult::NotFound unwrapped"),
        }
    }

    /// Return the payload as an Option.
    pub fn payload(self) -> Option<Vec<u8>> {
        match self {
            Self::Found(p) => Some(p),
            Self::NotFound => None,
        }
    }
}

/// A fetch request for a specific (name, version) pair.
#[derive(Debug, Clone)]
pub struct FetchRequest {
    pub name: String,
    pub version: Version,
}

impl FetchRequest {
    pub fn new(name: impl Into<String>, version: Version) -> Self {
        Self {
            name: name.into(),
            version,
        }
    }
}

/// Resolve a fetch request against a generic lookup function.
///
/// This thin adapter lets callers pass any lookup closure rather than
/// coupling directly to a `RegistryService`.
pub fn resolve<F>(request: &FetchRequest, lookup: F) -> FetchResult
where
    F: Fn(&str, &Version) -> Option<Vec<u8>>,
{
    match lookup(&request.name, &request.version) {
        Some(data) => FetchResult::Found(data),
        None => FetchResult::NotFound,
    }
}
