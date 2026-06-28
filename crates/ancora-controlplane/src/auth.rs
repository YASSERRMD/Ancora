use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing token")]
    MissingToken,
    #[error("invalid token")]
    InvalidToken,
}

#[derive(Debug, Clone)]
pub struct TokenAuth {
    valid_hashes: Vec<[u8; 32]>,
}

impl TokenAuth {
    pub fn new(tokens: &[&str]) -> Self {
        let valid_hashes = tokens.iter().map(|t| hash_token(t)).collect();
        TokenAuth { valid_hashes }
    }

    pub fn verify(&self, token: Option<&str>) -> Result<(), AuthError> {
        let token = token.ok_or(AuthError::MissingToken)?;
        if token.is_empty() {
            return Err(AuthError::MissingToken);
        }
        let h = hash_token(token);
        if self.valid_hashes.iter().any(|v| v == &h) {
            Ok(())
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}

fn hash_token(token: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hasher.finalize().into()
}
