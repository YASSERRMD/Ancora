use crate::registry_links::{link_count, registry_links};

#[test]
fn test_registry_links_nonempty() {
    assert!(link_count() > 0, "should have at least one registry link");
}

#[test]
fn test_all_links_have_urls_and_labels() {
    let links = registry_links();
    for link in &links {
        assert!(
            !link.url.is_empty(),
            "link {} should have a URL",
            link.label
        );
        assert!(!link.label.is_empty(), "link should have a label");
        assert!(
            !link.description.is_empty(),
            "link {} should have a description",
            link.label
        );
        assert!(
            link.url.starts_with("https://"),
            "link {} URL should use HTTPS",
            link.label
        );
    }
}
