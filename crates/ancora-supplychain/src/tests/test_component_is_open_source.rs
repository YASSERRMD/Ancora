#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};

    fn make_component(license: License) -> Component {
        Component::new(
            "id",
            "name",
            "1.0.0",
            ComponentKind::Library,
            license,
            "supplier",
            "sha256:00",
        )
    }

    #[test]
    fn test_mit_is_open_source() {
        assert!(make_component(License::Mit).is_open_source());
    }

    #[test]
    fn test_apache2_is_open_source() {
        assert!(make_component(License::Apache2).is_open_source());
    }

    #[test]
    fn test_gpl3_is_open_source() {
        assert!(make_component(License::Gpl3).is_open_source());
    }

    #[test]
    fn test_bsd2_is_open_source() {
        assert!(make_component(License::Bsd2).is_open_source());
    }

    #[test]
    fn test_bsd3_is_open_source() {
        assert!(make_component(License::Bsd3).is_open_source());
    }

    #[test]
    fn test_proprietary_is_not_open_source() {
        assert!(!make_component(License::Proprietary).is_open_source());
    }

    #[test]
    fn test_unknown_is_not_open_source() {
        assert!(!make_component(License::Unknown).is_open_source());
    }
}
