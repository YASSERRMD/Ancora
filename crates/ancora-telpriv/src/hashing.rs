//! Stable, one-way hashing for telemetry correlation IDs.
//!
//! Allows correlating telemetry records across a session without ever
//! exporting the raw value. The hash is deterministic so that replaying
//! the same events produces the same correlation IDs.
//!
//! We use FNV-1a (64-bit) - a zero-dependency, non-cryptographic hash
//! that is sufficient for correlation (not for security).

/// FNV-1a 64-bit basis and prime.
const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;

/// Compute a stable FNV-1a 64-bit hash of the input bytes.
pub fn fnv1a_64(data: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Hash a string value and return a hex string safe for telemetry export.
pub fn hash_value(value: &str) -> String {
    let h = fnv1a_64(value.as_bytes());
    format!("{:016x}", h)
}

/// Produce a salted hash using a fixed per-deployment salt.
/// The salt should be a deployment secret stored outside telemetry.
pub fn hash_with_salt(value: &str, salt: &str) -> String {
    let mut data = Vec::with_capacity(value.len() + salt.len() + 1);
    data.extend_from_slice(salt.as_bytes());
    data.push(b':');
    data.extend_from_slice(value.as_bytes());
    let h = fnv1a_64(&data);
    format!("{:016x}", h)
}

/// A correlation token: a hashed identifier that links telemetry records
/// without exposing the underlying value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CorrelationToken(String);

impl CorrelationToken {
    /// Create a correlation token from a raw value and a salt.
    pub fn from_raw(raw: &str, salt: &str) -> Self {
        CorrelationToken(hash_with_salt(raw, salt))
    }

    /// Return the opaque string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CorrelationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        assert_eq!(hash_value("user-123"), hash_value("user-123"));
    }

    #[test]
    fn different_inputs_different_hashes() {
        assert_ne!(hash_value("user-123"), hash_value("user-456"));
    }

    #[test]
    fn salt_changes_hash() {
        assert_ne!(
            hash_with_salt("user-123", "salt-a"),
            hash_with_salt("user-123", "salt-b")
        );
    }

    #[test]
    fn correlation_token_stable() {
        let t1 = CorrelationToken::from_raw("session-abc", "my-salt");
        let t2 = CorrelationToken::from_raw("session-abc", "my-salt");
        assert_eq!(t1, t2);
    }

    #[test]
    fn hash_output_is_hex_string() {
        let h = hash_value("test");
        assert_eq!(h.len(), 16);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
