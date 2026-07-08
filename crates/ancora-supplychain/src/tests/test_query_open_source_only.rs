#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::query::ComponentQuery;

    fn make_component(id: &str, license: License) -> Component {
        Component::new(
            id,
            "comp",
            "1.0.0",
            ComponentKind::Library,
            license,
            "vendor",
            "sha256:00",
        )
    }

    #[test]
    fn test_open_source_only_excludes_proprietary() {
        let components = [
            make_component("c1", License::Mit),
            make_component("c2", License::Proprietary),
            make_component("c3", License::Apache2),
        ];

        let results = ComponentQuery::new()
            .open_source_only()
            .run(components.iter());

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|c| c.is_open_source()));
    }

    #[test]
    fn test_open_source_only_excludes_unknown() {
        let components = [
            make_component("c1", License::Mit),
            make_component("c2", License::Unknown),
        ];

        let results = ComponentQuery::new()
            .open_source_only()
            .run(components.iter());

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "c1");
    }

    #[test]
    fn test_open_source_only_includes_all_oss_licenses() {
        let components = [
            make_component("c1", License::Mit),
            make_component("c2", License::Apache2),
            make_component("c3", License::Gpl3),
            make_component("c4", License::Bsd2),
            make_component("c5", License::Bsd3),
        ];

        let results = ComponentQuery::new()
            .open_source_only()
            .run(components.iter());

        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_open_source_only_all_proprietary_returns_empty() {
        let components = [
            make_component("c1", License::Proprietary),
            make_component("c2", License::Unknown),
        ];

        let results = ComponentQuery::new()
            .open_source_only()
            .run(components.iter());

        assert!(results.is_empty());
    }

    #[test]
    fn test_open_source_combined_with_kind_filter() {
        let components = [
            make_component("c1", License::Mit),
            make_component("c2", License::Apache2),
            make_component("c3", License::Proprietary),
        ];

        let results = ComponentQuery::new()
            .kind(ComponentKind::Library)
            .open_source_only()
            .run(components.iter());

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|c| c.is_open_source()));
    }
}
