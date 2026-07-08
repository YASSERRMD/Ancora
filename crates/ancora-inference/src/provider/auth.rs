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
                let token =
                    std::env::var(env_var).map_err(|_| format!("env var {env_var} not set"))?;
                Ok(Some((
                    "Authorization".to_owned(),
                    format!("Bearer {token}"),
                )))
            }
            Self::HeaderKey { header, env_var } => {
                let val =
                    std::env::var(env_var).map_err(|_| format!("env var {env_var} not set"))?;
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
        let auth = AuthStrategy::BearerToken {
            env_var: "NONEXISTENT_TEST_KEY_106".to_owned(),
        };
        assert!(auth.as_header().is_err());
    }

    #[test]
    fn none_auth_returns_no_header() {
        let auth = AuthStrategy::None;
        assert!(auth.as_header().unwrap().is_none());
        assert!(auth.as_query_param().is_none());
    }

    #[test]
    fn header_key_as_header_attaches_custom_header() {
        unsafe { std::env::set_var("ANCORA_TEST_HEADER_KEY_106", "sk-test-value") };
        let auth = AuthStrategy::HeaderKey {
            header: "X-Api-Key".to_owned(),
            env_var: "ANCORA_TEST_HEADER_KEY_106".to_owned(),
        };
        let (name, val) = auth.as_header().unwrap().unwrap();
        assert_eq!(name, "X-Api-Key");
        assert_eq!(val, "sk-test-value");
        unsafe { std::env::remove_var("ANCORA_TEST_HEADER_KEY_106") };
    }

    #[test]
    fn query_param_returns_name_and_value() {
        unsafe { std::env::set_var("ANCORA_TEST_QPARAM_106", "qp-token") };
        let auth = AuthStrategy::QueryParam {
            param: "api_key".to_owned(),
            env_var: "ANCORA_TEST_QPARAM_106".to_owned(),
        };
        let (name, val) = auth.as_query_param().unwrap();
        assert_eq!(name, "api_key");
        assert_eq!(val, "qp-token");
        // query-param auth produces no header
        assert!(auth.as_header().unwrap().is_none());
        unsafe { std::env::remove_var("ANCORA_TEST_QPARAM_106") };
    }

    #[test]
    fn bearer_token_attaches_correct_header_value() {
        unsafe { std::env::set_var("ANCORA_TEST_BEARER_106", "tok-abc") };
        let auth = AuthStrategy::BearerToken {
            env_var: "ANCORA_TEST_BEARER_106".to_owned(),
        };
        let (name, val) = auth.as_header().unwrap().unwrap();
        assert_eq!(name, "Authorization");
        assert_eq!(val, "Bearer tok-abc");
        unsafe { std::env::remove_var("ANCORA_TEST_BEARER_106") };
    }

    #[test]
    fn resolve_returns_token_value() {
        unsafe { std::env::set_var("ANCORA_TEST_RESOLVE_106", "resolved-secret") };
        let auth = AuthStrategy::BearerToken {
            env_var: "ANCORA_TEST_RESOLVE_106".to_owned(),
        };
        assert_eq!(auth.resolve().as_deref(), Some("resolved-secret"));
        unsafe { std::env::remove_var("ANCORA_TEST_RESOLVE_106") };
    }
}
