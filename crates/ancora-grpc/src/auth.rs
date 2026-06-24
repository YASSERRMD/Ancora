use tonic::{Request, Status};

/// Configuration for token-based authentication.
pub struct AuthConfig {
    pub token: String,
}

impl AuthConfig {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
}

/// Tonic interceptor that validates a Bearer token in the Authorization header.
#[derive(Clone)]
pub struct AuthInterceptor {
    expected: String,
}

impl AuthInterceptor {
    pub fn new(token: impl Into<String>) -> Self {
        Self { expected: token.into() }
    }
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
        let auth = req
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok());
        let provided = match auth {
            Some(v) => v,
            None => return Err(Status::unauthenticated("missing authorization header")),
        };
        let token = provided.strip_prefix("Bearer ").unwrap_or("");
        if token == self.expected {
            Ok(req)
        } else {
            Err(Status::unauthenticated("invalid token"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::service::Interceptor;

    fn req_with_token(token: &str) -> Request<()> {
        let mut req = Request::new(());
        req.metadata_mut().insert(
            "authorization",
            format!("Bearer {token}").parse().unwrap(),
        );
        req
    }

    #[test]
    fn valid_token_passes() {
        let mut interceptor = AuthInterceptor::new("secret");
        assert!(interceptor.call(req_with_token("secret")).is_ok());
    }

    #[test]
    fn wrong_token_returns_unauthenticated() {
        let mut interceptor = AuthInterceptor::new("secret");
        let err = interceptor.call(req_with_token("wrong")).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn missing_header_returns_unauthenticated() {
        let mut interceptor = AuthInterceptor::new("secret");
        let req = Request::new(());
        let err = interceptor.call(req).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn token_without_bearer_prefix_returns_unauthenticated() {
        let mut interceptor = AuthInterceptor::new("secret");
        let mut req = Request::new(());
        req.metadata_mut().insert("authorization", "secret".parse().unwrap());
        let err = interceptor.call(req).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
    }

    #[test]
    fn empty_token_returns_unauthenticated() {
        let mut interceptor = AuthInterceptor::new("secret");
        let err = interceptor.call(req_with_token("")).unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unauthenticated);
    }
}
