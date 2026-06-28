use crate::publish::PublishEntry;
use crate::search::SearchQuery;
use crate::service::{RegistryConfig, RegistryService};
use crate::versioning::Version;

fn populated_registry() -> RegistryService {
    let mut svc = RegistryService::new(RegistryConfig::default());
    let entries = [
        ("ancora-tool-alpha", "1.0.0"),
        ("ancora-tool-beta", "1.2.0"),
        ("ancora-connector-x", "0.5.0"),
        ("unrelated-package", "3.0.0"),
    ];
    for (name, ver) in entries {
        let version = Version::parse(ver).unwrap();
        svc.publish(PublishEntry::new(name, version, b"data".to_vec(), "ci"))
            .unwrap();
    }
    svc
}

#[test]
fn search_returns_matching_entries() {
    let svc = populated_registry();
    let hits = svc.search(&SearchQuery::new("ancora-tool"));
    assert_eq!(hits.len(), 2);
    let names: Vec<_> = hits.iter().map(|h| h.name.as_str()).collect();
    assert!(names.contains(&"ancora-tool-alpha"));
    assert!(names.contains(&"ancora-tool-beta"));
}

#[test]
fn search_is_case_insensitive() {
    let svc = populated_registry();
    let hits = svc.search(&SearchQuery::new("ANCORA-TOOL"));
    assert_eq!(hits.len(), 2);
}

#[test]
fn search_empty_term_returns_all() {
    let svc = populated_registry();
    let hits = svc.search(&SearchQuery::new(""));
    assert_eq!(hits.len(), 4);
}

#[test]
fn search_no_match_returns_empty() {
    let svc = populated_registry();
    let hits = svc.search(&SearchQuery::new("zzz-no-match"));
    assert!(hits.is_empty());
}
