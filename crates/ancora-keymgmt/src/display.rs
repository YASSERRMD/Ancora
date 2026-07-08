use crate::key::{CryptoKey, KeyPurpose};
use std::fmt;

impl fmt::Display for KeyPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyPurpose::Encryption => "ENCRYPTION",
            KeyPurpose::Signing => "SIGNING",
            KeyPurpose::Authentication => "AUTHENTICATION",
            KeyPurpose::KeyWrapping => "KEY_WRAPPING",
        };
        f.write_str(s)
    }
}

impl fmt::Display for CryptoKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Key({} v{} {} {} [{}])",
            self.id, self.version, self.algorithm, self.purpose, self.status
        )
    }
}
