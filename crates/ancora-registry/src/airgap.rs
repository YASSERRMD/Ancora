/// Controls whether the registry may make outbound network requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirgapMode {
    /// Normal operation: the registry may reach upstream sources.
    Online,
    /// Air-gapped operation: no outbound network calls are allowed.
    /// All operations must be served from local storage.
    AirGapped,
    /// Private mode: the registry accepts pushes from inside the network
    /// but does not forward requests to any upstream.
    Private,
}

impl Default for AirgapMode {
    fn default() -> Self {
        Self::Online
    }
}

impl AirgapMode {
    /// Returns true when outbound network access is allowed.
    pub fn allows_outbound(&self) -> bool {
        matches!(self, Self::Online)
    }

    /// Returns true when this registry operates in an isolated (air-gapped or private) mode.
    pub fn is_isolated(&self) -> bool {
        !self.allows_outbound()
    }
}

/// A guard that enforces the air-gap policy when wrapping an operation that
/// would otherwise require network access.
pub struct AirgapGuard<'a> {
    mode: &'a AirgapMode,
}

impl<'a> AirgapGuard<'a> {
    pub fn new(mode: &'a AirgapMode) -> Self {
        Self { mode }
    }

    /// Attempt to perform a network-requiring operation.
    ///
    /// In air-gapped or private mode the supplied closure is NOT called and
    /// an `Err` is returned instead.
    pub fn try_outbound<T, F>(&self, f: F) -> Result<T, AirgapError>
    where
        F: FnOnce() -> T,
    {
        if self.mode.allows_outbound() {
            Ok(f())
        } else {
            Err(AirgapError::NetworkDisabled)
        }
    }
}

/// Errors arising from air-gap policy violations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirgapError {
    /// A network call was attempted but the registry is in isolated mode.
    NetworkDisabled,
}

impl std::fmt::Display for AirgapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkDisabled => write!(f, "network access is disabled in this registry mode"),
        }
    }
}
