/// Authentication strategy attached to a provider profile.
#[derive(Debug, Clone)]
pub enum AuthStrategy {
    /// Adds `Authorization: Bearer <token>` resolved from an env var.
    BearerToken { env_var: String },
    /// Adds an arbitrary header resolved from an env var.
    HeaderKey { header: String, env_var: String },
    /// Appends a query parameter resolved from an env var.
    QueryParam { param: String, env_var: String },
    /// No authentication (local servers, public endpoints).
    None,
}

impl AuthStrategy {
    /// Resolve the raw credential value from the environment, if applicable.
    pub fn resolve(&self) -> Option<String> {
        match self {
            Self::BearerToken { env_var } => std::env::var(env_var).ok(),
            Self::HeaderKey { env_var, .. } => std::env::var(env_var).ok(),
            Self::QueryParam { env_var, .. } => std::env::var(env_var).ok(),
            Self::None => None,
        }
    }

    /// Return the header name and value for header-based auth, or `None`.
    pub fn as_header(&self) -> Result<Option<(String, String)>, String> {
        match self {
            Self::BearerToken { env_var } => {
                let token = std::env::var(env_var)
                    .map_err(|_| format!("env var {env_var} not set"))?;
                Ok(Some(("Authorization".to_owned(), format!("Bearer {token}"))))
            }
            Self::HeaderKey { header, env_var } => {
                let val = std::env::var(env_var)
                    .map_err(|_| format!("env var {env_var} not set"))?;
                Ok(Some((header.clone(), val)))
            }
            Self::QueryParam { .. } | Self::None => Ok(None),
        }
    }

    /// Return `(param_name, value)` for query-param auth, or `None`.
    pub fn as_query_param(&self) -> Option<(String, String)> {
        if let Self::QueryParam { param, env_var } = self {
            let val = std::env::var(env_var).ok()?;
            Some((param.clone(), val))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_token_as_header_missing_env_returns_err() {
        let auth = AuthStrategy::BearerToken { env_var: "NONEXISTENT_TEST_KEY_106".to_owned() };
        assert!(auth.as_header().is_err());
    }

    #[test]
    fn none_auth_returns_no_header() {
        let auth = AuthStrategy::None;
        assert!(auth.as_header().unwrap().is_none());
        assert!(auth.as_query_param().is_none());
    }
}
