use crate::error::SecretError;

pub fn validate_path(path: &str) -> Result<(), SecretError> {
    if path.is_empty() {
        return Err(SecretError::InvalidPath("path must not be empty".into()));
    }
    if path.contains(' ') {
        return Err(SecretError::InvalidPath(format!(
            "path '{}' must not contain spaces",
            path
        )));
    }
    if path.len() > 256 {
        return Err(SecretError::InvalidPath(format!(
            "path '{}' exceeds max length 256",
            path
        )));
    }
    for c in path.chars() {
        if !c.is_ascii_alphanumeric() && !"/.-_".contains(c) {
            return Err(SecretError::InvalidPath(format!(
                "path '{}' contains invalid character '{}'",
                path, c
            )));
        }
    }
    if path.starts_with('/') || path.ends_with('/') {
        return Err(SecretError::InvalidPath(format!(
            "path '{}' must not start or end with '/'",
            path
        )));
    }
    Ok(())
}
