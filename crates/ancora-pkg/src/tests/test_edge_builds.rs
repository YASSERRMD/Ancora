use crate::edge_template::{EdgeArch, EdgeConfig, EdgeConstraints, EdgeError, EdgeTemplate};

#[test]
fn test_edge_basic_render() {
    let config = EdgeConfig::new("ancora-edge", "1.0.0", EdgeArch::X86_64);
    let tmpl = EdgeTemplate::render(config).expect("should render");
    assert!(tmpl.build_spec.contains("ancora-edge"));
    assert!(tmpl.build_spec.contains("x86_64-unknown-linux-musl"));
}

#[test]
fn test_edge_static_binary() {
    let config = EdgeConfig::new("static-edge", "1.0.0", EdgeArch::Aarch64);
    assert!(config.static_binary, "must default to static binary");
    assert!(config.strip_symbols, "must default to stripped symbols");
    let tmpl = EdgeTemplate::render(config).expect("should render");
    assert!(tmpl.contains_in_build("static_binary: true"));
    assert!(tmpl.contains_in_build("strip_symbols: true"));
}

#[test]
fn test_edge_arch_triples() {
    assert_eq!(EdgeArch::X86_64.triple(), "x86_64-unknown-linux-musl");
    assert_eq!(EdgeArch::Aarch64.triple(), "aarch64-unknown-linux-musl");
    assert_eq!(EdgeArch::Armv7.triple(), "armv7-unknown-linux-musleabihf");
    assert_eq!(EdgeArch::RiscV64.triple(), "riscv64gc-unknown-linux-gnu");
}

#[test]
fn test_edge_runtime_security() {
    let config = EdgeConfig::new("sec-edge", "1.0.0", EdgeArch::X86_64);
    let tmpl = EdgeTemplate::render(config).expect("should render");
    assert!(tmpl.runtime_config.contains("tls: required"), "TLS must be required at the edge");
    assert!(tmpl.runtime_config.contains("no_root: true"), "must run without root");
    assert!(tmpl.runtime_config.contains("audit_log: enabled"), "audit log must be enabled");
}

#[test]
fn test_edge_binary_name() {
    let config = EdgeConfig::new("my-edge", "2.3.1", EdgeArch::Armv7);
    let tmpl = EdgeTemplate::render(config).expect("should render");
    let name = tmpl.binary_name();
    assert_eq!(name, "my-edge-2.3.1-armv7");
}

#[test]
fn test_edge_with_features() {
    let config = EdgeConfig::new("feature-edge", "1.0.0", EdgeArch::X86_64)
        .with_feature("offline-mode")
        .with_feature("compact-log");
    let tmpl = EdgeTemplate::render(config).expect("should render");
    assert!(tmpl.contains_in_build("offline-mode"));
    assert!(tmpl.contains_in_build("compact-log"));
}

#[test]
fn test_edge_resource_constraints_in_runtime() {
    let constraints = EdgeConstraints {
        max_memory_mb: 128,
        max_cpu_percent: 25,
        storage_limit_mb: 256,
    };
    let config = EdgeConfig::new("small-edge", "1.0.0", EdgeArch::Armv7)
        .with_constraints(constraints);
    let tmpl = EdgeTemplate::render(config).expect("should render");
    assert!(tmpl.runtime_config.contains("max_memory_mb: 128"));
    assert!(tmpl.runtime_config.contains("max_cpu_percent: 25"));
}

#[test]
fn test_edge_empty_product_fails() {
    let config = EdgeConfig::new("", "1.0.0", EdgeArch::X86_64);
    let err = EdgeTemplate::render(config).unwrap_err();
    assert!(matches!(err, EdgeError::InvalidConfig(_)));
}

#[test]
fn test_edge_empty_version_fails() {
    let config = EdgeConfig::new("agent", "", EdgeArch::X86_64);
    let err = EdgeTemplate::render(config).unwrap_err();
    assert!(matches!(err, EdgeError::InvalidConfig(_)));
}
