#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};

    #[test]
    fn test_new_stores_id() {
        let c = Component::new(
            "comp-1",
            "openssl",
            "3.0.0",
            ComponentKind::Library,
            License::Mit,
            "acme-corp",
            "sha256:abc123",
        );
        assert_eq!(c.id, "comp-1");
    }

    #[test]
    fn test_new_stores_name() {
        let c = Component::new(
            "comp-2",
            "nginx",
            "1.25.0",
            ComponentKind::Binary,
            License::Bsd2,
            "nginx-inc",
            "sha256:def456",
        );
        assert_eq!(c.name, "nginx");
    }

    #[test]
    fn test_new_stores_version() {
        let c = Component::new(
            "comp-3",
            "ubuntu",
            "22.04",
            ComponentKind::Container,
            License::Proprietary,
            "canonical",
            "sha256:789abc",
        );
        assert_eq!(c.version, "22.04");
    }

    #[test]
    fn test_new_stores_kind() {
        let c = Component::new(
            "comp-4",
            "libc",
            "2.38",
            ComponentKind::OsPackage,
            License::Gpl3,
            "gnu",
            "sha256:aaaaaa",
        );
        assert!(matches!(c.kind, ComponentKind::OsPackage));
    }

    #[test]
    fn test_new_stores_license() {
        let c = Component::new(
            "comp-5",
            "react",
            "18.2.0",
            ComponentKind::Framework,
            License::Apache2,
            "meta",
            "sha256:bbbbbb",
        );
        assert!(matches!(c.license, License::Apache2));
    }

    #[test]
    fn test_new_stores_supplier() {
        let c = Component::new(
            "comp-6",
            "svc-auth",
            "0.1.0",
            ComponentKind::Service,
            License::Unknown,
            "internal-team",
            "sha256:cccccc",
        );
        assert_eq!(c.supplier, "internal-team");
    }

    #[test]
    fn test_new_stores_digest() {
        let c = Component::new(
            "comp-7",
            "curl",
            "8.0.0",
            ComponentKind::Binary,
            License::Mit,
            "haxx",
            "sha256:deadbeef",
        );
        assert_eq!(c.digest, "sha256:deadbeef");
    }
}
