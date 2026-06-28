use crate::provider_template::{
    GenerateRequest, Message, MyProvider, ProviderAdapter, ProviderError, Role,
};

#[test]
fn provider_id_is_correct() {
    let p = MyProvider::new("test-key");
    assert_eq!(p.provider_id(), "my-provider");
}

#[test]
fn list_models_nonempty() {
    let p = MyProvider::new("test-key");
    assert!(!p.list_models().is_empty());
}

#[test]
fn generate_echoes_last_message() {
    let p = MyProvider::new("test-key");
    let req = GenerateRequest {
        model: "my-provider-v1".to_string(),
        messages: vec![
            Message { role: Role::System, content: "You are helpful.".to_string() },
            Message { role: Role::User, content: "hello world".to_string() },
        ],
        max_tokens: Some(32),
        temperature: Some(0.0),
    };
    let resp = p.generate(req).expect("generate must succeed");
    assert!(resp.content.contains("hello world"));
    assert_eq!(resp.model, "my-provider-v1");
    assert!(!resp.truncated);
}

#[test]
fn generate_unknown_model_returns_error() {
    let p = MyProvider::new("test-key");
    let req = GenerateRequest {
        model: "nonexistent-model".to_string(),
        messages: vec![Message { role: Role::User, content: "hi".to_string() }],
        max_tokens: None,
        temperature: None,
    };
    match p.generate(req) {
        Err(ProviderError::ModelNotFound(m)) => assert_eq!(m, "nonexistent-model"),
        other => panic!("expected ModelNotFound, got {other:?}"),
    }
}

#[test]
fn generate_empty_messages_produces_output() {
    let p = MyProvider::new("test-key");
    let req = GenerateRequest {
        model: "my-provider-v1".to_string(),
        messages: vec![],
        max_tokens: None,
        temperature: None,
    };
    let resp = p.generate(req).expect("generate must succeed for empty messages");
    // Echo of empty string still produces a response
    assert!(!resp.content.is_empty());
}

#[test]
fn provider_error_display() {
    assert!(!ProviderError::RateLimited.to_string().is_empty());
    assert!(!ProviderError::AuthFailed.to_string().is_empty());
    assert!(ProviderError::Unavailable("svc".into()).to_string().contains("svc"));
}
