use crate::selection::{parse_backends, ExporterBackend, ExporterSelection, SelectionError};
use crate::selfhosted::ResidencyPolicy;

#[test]
fn test_parse_backends_single() {
    let backends = parse_backends("otlp").unwrap();
    assert_eq!(backends, vec![ExporterBackend::Otlp]);
}

#[test]
fn test_parse_backends_multiple() {
    let backends = parse_backends("otlp,prometheus,langfuse").unwrap();
    assert_eq!(backends.len(), 3);
    assert!(backends.contains(&ExporterBackend::Otlp));
    assert!(backends.contains(&ExporterBackend::Prometheus));
    assert!(backends.contains(&ExporterBackend::Langfuse));
}

#[test]
fn test_parse_backends_unknown_returns_error() {
    let result = parse_backends("unknown_backend");
    assert!(result.is_err());
    matches!(result.unwrap_err(), SelectionError::InvalidConfig { .. });
}

#[test]
fn test_selection_add_internal_backend_unrestricted() {
    let mut sel = ExporterSelection::new(ResidencyPolicy::Unrestricted);
    assert!(sel.add_backend(ExporterBackend::Otlp).is_ok());
    assert!(sel.has_backend(&ExporterBackend::Otlp));
}

#[test]
fn test_selection_add_external_backend_unrestricted() {
    let mut sel = ExporterSelection::new(ResidencyPolicy::Unrestricted);
    assert!(sel.add_backend(ExporterBackend::Datadog).is_ok());
    assert!(sel.has_backend(&ExporterBackend::Datadog));
}

#[test]
fn test_selection_add_external_backend_self_hosted_blocked() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal".to_string()]);
    let mut sel = ExporterSelection::new(policy);
    let result = sel.add_backend(ExporterBackend::Langfuse);
    assert!(result.is_err());
    matches!(
        result.unwrap_err(),
        SelectionError::ExternalBackendForbidden { .. }
    );
}

#[test]
fn test_selection_add_internal_backend_self_hosted_allowed() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal".to_string()]);
    let mut sel = ExporterSelection::new(policy);
    assert!(sel.add_backend(ExporterBackend::Otlp).is_ok());
    assert!(sel.add_backend(ExporterBackend::Prometheus).is_ok());
    assert!(sel.add_backend(ExporterBackend::GrafanaTempo).is_ok());
}

#[test]
fn test_selection_no_duplicates() {
    let mut sel = ExporterSelection::new(ResidencyPolicy::Unrestricted);
    sel.add_backend(ExporterBackend::Otlp).unwrap();
    sel.add_backend(ExporterBackend::Otlp).unwrap();
    assert_eq!(sel.active_count(), 1);
}

#[test]
fn test_backend_is_external() {
    assert!(ExporterBackend::Langfuse.is_external());
    assert!(ExporterBackend::Phoenix.is_external());
    assert!(ExporterBackend::Datadog.is_external());
    assert!(!ExporterBackend::Otlp.is_external());
    assert!(!ExporterBackend::Prometheus.is_external());
    assert!(!ExporterBackend::GrafanaTempo.is_external());
}
