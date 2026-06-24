use ancora_cli_lib::studio::StudioServer;
use ancora_core::journal::MemoryStore;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

fn make_server() -> StudioServer {
    let store = Arc::new(MemoryStore::new());
    StudioServer::bind(0, store).unwrap()
}

fn http_get(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let req = format!("GET {} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n", path);
    stream.write_all(req.as_bytes()).unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).unwrap();
    resp
}

fn http_post(port: u16, path: &str) -> String {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
    let req = format!(
        "POST {} HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        path
    );
    stream.write_all(req.as_bytes()).unwrap();
    let mut resp = String::new();
    stream.read_to_string(&mut resp).unwrap();
    resp
}

#[test]
fn studio_server_binds_to_os_assigned_port() {
    let server = make_server();
    assert!(server.port() > 0);
}
