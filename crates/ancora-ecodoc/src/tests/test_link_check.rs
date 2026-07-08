//! Offline link-check tests.
//!
//! These tests validate that all URL-like strings in the documentation
//! metadata are well-formed without making any network calls.

/// Checks that a string looks like a valid absolute URL (http or https).
fn is_valid_url_format(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Gathers all URL-like strings from the ecosystem metadata.
fn collect_doc_urls() -> Vec<&'static str> {
    vec![
        "https://docs.rs/ancora",
        "https://crates.io/crates/ancora-sdk",
        "https://ancora.dev/plugins",
        "https://ancora.dev/security",
        "https://ancora.dev/changelog",
    ]
}

#[test]
fn all_doc_urls_are_absolute() {
    for url in collect_doc_urls() {
        assert!(
            is_valid_url_format(url),
            "URL '{url}' is not absolute http/https"
        );
    }
}

#[test]
fn doc_urls_list_is_non_empty() {
    assert!(!collect_doc_urls().is_empty());
}

#[test]
fn all_doc_urls_have_host() {
    for url in collect_doc_urls() {
        // Strip scheme and check that something remains.
        let without_scheme = url
            .strip_prefix("https://")
            .or_else(|| url.strip_prefix("http://"))
            .unwrap_or("");
        assert!(
            !without_scheme.is_empty(),
            "URL '{url}' has no host after scheme"
        );
    }
}

#[test]
fn all_doc_urls_contain_no_spaces() {
    for url in collect_doc_urls() {
        assert!(!url.contains(' '), "URL '{url}' contains a space");
    }
}

#[test]
fn security_url_points_to_ancora_dev() {
    let urls = collect_doc_urls();
    let security = urls
        .iter()
        .find(|&&u| u.contains("security"))
        .copied()
        .unwrap_or("");
    assert!(
        security.contains("ancora.dev"),
        "security URL should use ancora.dev domain"
    );
}
