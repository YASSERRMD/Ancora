use crate::cli::{CliOutput, run_command, parse_command, CliCommand};
use crate::service::{RegistryConfig, RegistryService};
use crate::versioning::Version;

#[test]
fn cli_publish_then_fetch() {
    let mut svc = RegistryService::new(RegistryConfig::default());

    let publish_cmd = CliCommand::Publish {
        name: "cli-tool".to_string(),
        version: Version::new(1, 0, 0),
        payload: b"cli-payload".to_vec(),
        publisher: "dev".to_string(),
        signature: None,
    };
    let out = run_command(&mut svc, publish_cmd);
    assert!(out.is_ok(), "publish should succeed");

    let fetch_cmd = CliCommand::Fetch {
        name: "cli-tool".to_string(),
        version: Version::new(1, 0, 0),
    };
    let out = run_command(&mut svc, fetch_cmd);
    assert_eq!(out, CliOutput::Payload(b"cli-payload".to_vec()));
}

#[test]
fn cli_fetch_missing_returns_error() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let cmd = CliCommand::Fetch {
        name: "ghost".to_string(),
        version: Version::new(9, 9, 9),
    };
    let out = run_command(&mut svc, cmd);
    assert!(matches!(out, CliOutput::Err(_)));
}

#[test]
fn cli_search_returns_lines() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let _ = run_command(&mut svc, CliCommand::Publish {
        name: "searchable-tool".to_string(),
        version: Version::new(1, 0, 0),
        payload: b"x".to_vec(),
        publisher: "ci".to_string(),
        signature: None,
    });
    let out = run_command(&mut svc, CliCommand::Search { term: "searchable".to_string() });
    assert!(matches!(out, CliOutput::Lines(_)));
    if let CliOutput::Lines(lines) = out {
        assert!(!lines.is_empty());
    }
}

#[test]
fn parse_command_publish_valid() {
    let cmd = parse_command("publish my-tool 1.2.3 alice").unwrap();
    assert!(matches!(cmd, CliCommand::Publish { .. }));
}

#[test]
fn parse_command_fetch_valid() {
    let cmd = parse_command("fetch my-tool 1.2.3").unwrap();
    assert!(matches!(cmd, CliCommand::Fetch { .. }));
}

#[test]
fn parse_command_unknown_returns_err() {
    assert!(parse_command("invalid command here blah").is_err());
}

#[test]
fn cli_versions_command() {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let _ = run_command(&mut svc, CliCommand::Publish {
        name: "versioned-tool".to_string(),
        version: Version::new(1, 0, 0),
        payload: b"v1".to_vec(),
        publisher: "ci".to_string(),
        signature: None,
    });
    let out = run_command(&mut svc, CliCommand::Versions { name: "versioned-tool".to_string() });
    if let CliOutput::Lines(lines) = out {
        assert!(lines.contains(&"1.0.0".to_string()));
    } else {
        panic!("expected Lines output");
    }
}
