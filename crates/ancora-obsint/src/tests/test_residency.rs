use crate::selfhosted::{is_export_permitted, ResidencyError, ResidencyPolicy};
use crate::selection::{ExporterBackend, ExporterSelection, SelectionError};

#[test]
fn test_residency_blocks_langfuse_in_self_hosted() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal.corp".to_string()]);
    let mut sel = ExporterSelection::new(policy);
    let err = sel.add_backend(ExporterBackend::Langfuse).unwrap_err();
    assert_eq!(
        err,
        SelectionError::ExternalBackendForbidden {
            backend: "langfuse".to_string()
        }
    );
}

#[test]
fn test_residency_blocks_datadog_in_self_hosted() {
    let policy = ResidencyPolicy::self_hosted(vec![]);
    let mut sel = ExporterSelection::new(policy);
    let err = sel.add_backend(ExporterBackend::Datadog).unwrap_err();
    assert_eq!(
        err,
        SelectionError::ExternalBackendForbidden {
            backend: "datadog".to_string()
        }
    );
}

#[test]
fn test_residency_blocks_phoenix_in_self_hosted() {
    let policy = ResidencyPolicy::self_hosted(vec![]);
    let mut sel = ExporterSelection::new(policy);
    assert!(sel.add_backend(ExporterBackend::Phoenix).is_err());
}

#[test]
fn test_residency_endpoint_check_boundary() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://10.0.0.1".to_string()]);
    // Exact prefix matches
    assert!(policy.check_endpoint("http://10.0.0.1:4317/otlp").is_ok());
    // Different IP - blocked
    assert!(policy.check_endpoint("http://10.0.0.2:4317/otlp").is_err());
}

#[test]
fn test_is_export_permitted_with_multiple_prefixes() {
    let policy = ResidencyPolicy::self_hosted(vec![
        "http://host-a".to_string(),
        "http://host-b".to_string(),
    ]);
    assert!(is_export_permitted(&policy, "http://host-a/api"));
    assert!(is_export_permitted(&policy, "http://host-b/api"));
    assert!(!is_export_permitted(&policy, "http://host-c/api"));
}

#[test]
fn test_unrestricted_allows_all_backends() {
    let mut sel = ExporterSelection::new(ResidencyPolicy::Unrestricted);
    assert!(sel.add_backend(ExporterBackend::Langfuse).is_ok());
    assert!(sel.add_backend(ExporterBackend::Datadog).is_ok());
    assert!(sel.add_backend(ExporterBackend::Phoenix).is_ok());
    assert!(sel.add_backend(ExporterBackend::Otlp).is_ok());
    assert!(sel.add_backend(ExporterBackend::Prometheus).is_ok());
}

#[test]
fn test_residency_error_display() {
    let err = ResidencyError::ExternalEndpointBlocked {
        endpoint: "https://example.com".to_string(),
    };
    let s = err.to_string();
    assert!(s.contains("https://example.com"));
    assert!(s.contains("blocks"));
}

#[test]
fn test_selection_error_display() {
    let err = SelectionError::ExternalBackendForbidden {
        backend: "datadog".to_string(),
    };
    let s = err.to_string();
    assert!(s.contains("datadog"));
    assert!(s.contains("forbidden"));
}
