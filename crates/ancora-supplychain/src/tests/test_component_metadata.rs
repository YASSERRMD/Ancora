#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};

    fn make_component() -> Component {
        Component::new(
            "comp-meta",
            "mylib",
            "2.0.0",
            ComponentKind::Library,
            License::Apache2,
            "vendor",
            "sha256:meta",
        )
    }

    #[test]
    fn test_with_metadata_stores_single_key() {
        let c = make_component().with_metadata("cpe", "cpe:/a:vendor:mylib:2.0.0");
        assert_eq!(c.metadata.get("cpe"), Some(&"cpe:/a:vendor:mylib:2.0.0".to_string()));
    }

    #[test]
    fn test_with_metadata_stores_multiple_keys() {
        let c = make_component()
            .with_metadata("cpe", "cpe:/a:vendor:mylib:2.0.0")
            .with_metadata("purl", "pkg:cargo/mylib@2.0.0");
        assert_eq!(c.metadata.get("purl"), Some(&"pkg:cargo/mylib@2.0.0".to_string()));
        assert_eq!(c.metadata.get("cpe"), Some(&"cpe:/a:vendor:mylib:2.0.0".to_string()));
    }

    #[test]
    fn test_with_metadata_missing_key_returns_none() {
        let c = make_component().with_metadata("key", "value");
        assert!(c.metadata.get("nonexistent").is_none());
    }

    #[test]
    fn test_with_metadata_overwrites_existing_key() {
        let c = make_component()
            .with_metadata("key", "first")
            .with_metadata("key", "second");
        assert_eq!(c.metadata.get("key"), Some(&"second".to_string()));
    }
}
