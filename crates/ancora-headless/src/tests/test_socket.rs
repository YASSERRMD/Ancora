use crate::socket::{
    ApiMethod, SocketApiHandler, SocketConfig, SocketRequest, DEFAULT_SOCKET_PATH,
};

#[test]
fn test_socket_api_serves_ping() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    let req = SocketRequest::new(1, "ping");
    let resp = handler.handle(req);
    assert!(resp.ok);
    assert_eq!(resp.body, "pong");
}

#[test]
fn test_socket_api_serves_run() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    let req = SocketRequest::new(2, "run").with_param("prompt", "hello");
    let resp = handler.handle(req);
    assert!(resp.ok);
    assert!(resp.body.contains("hello"));
}

#[test]
fn test_socket_api_run_missing_prompt_returns_error() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    let req = SocketRequest::new(3, "run");
    let resp = handler.handle(req);
    assert!(!resp.ok);
}

#[test]
fn test_socket_api_status() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    let req = SocketRequest::new(4, "status");
    let resp = handler.handle(req);
    assert!(resp.ok);
    assert_eq!(resp.body, "ready");
}

#[test]
fn test_socket_api_list_models() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    handler.loaded_models.push("model-a".to_string());
    let req = SocketRequest::new(5, "list_models");
    let resp = handler.handle(req);
    assert!(resp.ok);
    assert!(resp.body.contains("model-a"));
}

#[test]
fn test_socket_api_unknown_method() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    let req = SocketRequest::new(6, "nonexistent");
    let resp = handler.handle(req);
    assert!(!resp.ok);
}

#[test]
fn test_api_method_from_str() {
    assert_eq!(ApiMethod::from("ping"), ApiMethod::Ping);
    assert_eq!(ApiMethod::from("run"), ApiMethod::Run);
}

#[test]
fn test_socket_config_defaults() {
    let cfg = SocketConfig::default();
    assert!(cfg.is_tmpfs());
    assert!(!cfg.is_abstract());
}

#[test]
fn test_socket_request_count_tracked() {
    let mut handler = SocketApiHandler::new(DEFAULT_SOCKET_PATH);
    handler.handle(SocketRequest::new(1, "ping"));
    handler.handle(SocketRequest::new(2, "status"));
    assert_eq!(handler.request_count(), 2);
}
