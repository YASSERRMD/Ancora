//! Plugin safety model and sandboxing policies.
//!
//! Documents the trust boundaries enforced by the Ancora runtime
//! when loading and executing third-party plugins.

/// Trust level assigned to a plugin at load time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Untrusted: loaded in strict sandbox with minimal capabilities.
    Untrusted = 0,
    /// Community: vetted by the community catalog.
    Community = 1,
    /// Verified: cryptographically signed by a known publisher.
    Verified = 2,
    /// Core: first-party Ancora plugin.
    Core = 3,
}

/// Capability that a plugin may request.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    NetworkAccess,
    FileSystemRead,
    FileSystemWrite,
    SubprocessExecution,
    SecretAccess,
}

/// Policies per trust level.
pub fn allowed_capabilities(level: TrustLevel) -> Vec<Capability> {
    match level {
        TrustLevel::Untrusted => vec![],
        TrustLevel::Community => vec![Capability::FileSystemRead],
        TrustLevel::Verified => vec![
            Capability::FileSystemRead,
            Capability::FileSystemWrite,
            Capability::NetworkAccess,
        ],
        TrustLevel::Core => vec![
            Capability::FileSystemRead,
            Capability::FileSystemWrite,
            Capability::NetworkAccess,
            Capability::SubprocessExecution,
            Capability::SecretAccess,
        ],
    }
}

/// Returns whether a capability is permitted for a given trust level.
pub fn is_permitted(level: TrustLevel, cap: &Capability) -> bool {
    allowed_capabilities(level).contains(cap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untrusted_has_no_capabilities() {
        assert!(allowed_capabilities(TrustLevel::Untrusted).is_empty());
    }

    #[test]
    fn core_has_all_capabilities() {
        let caps = allowed_capabilities(TrustLevel::Core);
        assert!(caps.contains(&Capability::SecretAccess));
        assert!(caps.contains(&Capability::SubprocessExecution));
    }

    #[test]
    fn community_cannot_write() {
        assert!(!is_permitted(
            TrustLevel::Community,
            &Capability::FileSystemWrite
        ));
    }
}
