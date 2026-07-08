use crate::airgap::{AirgapError, AirgapGuard, AirgapMode};
use crate::fetch::FetchResult;
use crate::publish::PublishEntry;
use crate::service::{RegistryConfig, RegistryService};
use crate::versioning::Version;

#[test]
fn online_mode_allows_outbound() {
    let mode = AirgapMode::Online;
    let guard = AirgapGuard::new(&mode);
    let result = guard.try_outbound(|| 42u32);
    assert_eq!(result, Ok(42));
}

#[test]
fn airgapped_mode_blocks_outbound() {
    let mode = AirgapMode::AirGapped;
    let guard = AirgapGuard::new(&mode);
    let result = guard.try_outbound(|| 42u32);
    assert_eq!(result, Err(AirgapError::NetworkDisabled));
}

#[test]
fn private_mode_blocks_outbound() {
    let mode = AirgapMode::Private;
    let guard = AirgapGuard::new(&mode);
    let result = guard.try_outbound(|| "hello");
    assert_eq!(result, Err(AirgapError::NetworkDisabled));
}

#[test]
fn airgapped_registry_serves_local_entries() {
    // Even in air-gapped mode, entries already in the local store are fetchable.
    let cfg = RegistryConfig {
        airgap_mode: AirgapMode::AirGapped,
        ..Default::default()
    };
    let mut svc = RegistryService::new(cfg);
    let version = Version::new(1, 0, 0);
    svc.publish(PublishEntry::new(
        "local-tool",
        version.clone(),
        b"payload".to_vec(),
        "ci",
    ))
    .unwrap();

    assert_eq!(
        svc.fetch("local-tool", &version),
        FetchResult::Found(b"payload".to_vec())
    );
}

#[test]
fn airgap_mode_is_isolated() {
    assert!(AirgapMode::AirGapped.is_isolated());
    assert!(AirgapMode::Private.is_isolated());
    assert!(!AirgapMode::Online.is_isolated());
}
