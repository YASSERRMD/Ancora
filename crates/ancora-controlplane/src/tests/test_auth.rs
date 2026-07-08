#[cfg(test)]
mod tests {
    use crate::api::runs::RunsApi;
    use crate::auth::{AuthError, TokenAuth};
    use crate::model::RunPriority;
    use crate::store::ControlPlaneStore;

    #[test]
    fn auth_rejects_missing_token() {
        let mut store = ControlPlaneStore::new();
        let auth = TokenAuth::new(&["secret"]);
        let mut api = RunsApi::new(&mut store, &auth);
        let err = api.create(None, "t1", RunPriority::Normal).unwrap_err();
        matches!(
            err,
            crate::api::runs::RunsApiError::Auth(AuthError::MissingToken)
        );
    }

    #[test]
    fn auth_rejects_wrong_token() {
        let mut store = ControlPlaneStore::new();
        let auth = TokenAuth::new(&["correct-token"]);
        let mut api = RunsApi::new(&mut store, &auth);
        let err = api
            .create(Some("wrong-token"), "t1", RunPriority::Normal)
            .unwrap_err();
        matches!(
            err,
            crate::api::runs::RunsApiError::Auth(AuthError::InvalidToken)
        );
    }

    #[test]
    fn auth_accepts_valid_token() {
        let mut store = ControlPlaneStore::new();
        let auth = TokenAuth::new(&["my-token"]);
        let mut api = RunsApi::new(&mut store, &auth);
        let run = api
            .create(Some("my-token"), "t1", RunPriority::Normal)
            .unwrap();
        assert!(!run.id.is_empty());
    }

    #[test]
    fn auth_multi_token_any_valid() {
        let mut store = ControlPlaneStore::new();
        let auth = TokenAuth::new(&["token-a", "token-b"]);
        let mut api = RunsApi::new(&mut store, &auth);
        api.create(Some("token-a"), "t1", RunPriority::Normal)
            .unwrap();
        api.create(Some("token-b"), "t1", RunPriority::Normal)
            .unwrap();
    }
}
