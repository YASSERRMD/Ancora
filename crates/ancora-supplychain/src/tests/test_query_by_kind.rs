#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::query::ComponentQuery;

    fn make_component(id: &str, kind: ComponentKind) -> Component {
        Component::new(
            id,
            "comp",
            "1.0.0",
            kind,
            License::Mit,
            "vendor",
            "sha256:00",
        )
    }

    #[test]
    fn test_query_kind_library_filters_only_library() {
        let components = vec![
            make_component("c1", ComponentKind::Library),
            make_component("c2", ComponentKind::Binary),
            make_component("c3", ComponentKind::Library),
            make_component("c4", ComponentKind::Container),
        ];

        let results = ComponentQuery::new()
            .kind(ComponentKind::Library)
            .run(components.iter());

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|c| matches!(c.kind, ComponentKind::Library)));
    }

    #[test]
    fn test_query_kind_binary_filters_only_binary() {
        let components = vec![
            make_component("c1", ComponentKind::Library),
            make_component("c2", ComponentKind::Binary),
        ];

        let results = ComponentQuery::new()
            .kind(ComponentKind::Binary)
            .run(components.iter());

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "c2");
    }

    #[test]
    fn test_query_kind_no_match_returns_empty() {
        let components = vec![
            make_component("c1", ComponentKind::Library),
            make_component("c2", ComponentKind::Binary),
        ];

        let results = ComponentQuery::new()
            .kind(ComponentKind::Service)
            .run(components.iter());

        assert!(results.is_empty());
    }

    #[test]
    fn test_query_no_filter_returns_all() {
        let components = vec![
            make_component("c1", ComponentKind::Library),
            make_component("c2", ComponentKind::Binary),
            make_component("c3", ComponentKind::Container),
        ];

        let results = ComponentQuery::new().run(components.iter());
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_query_kind_container() {
        let components = vec![
            make_component("c1", ComponentKind::Container),
            make_component("c2", ComponentKind::Library),
            make_component("c3", ComponentKind::Container),
        ];

        let results = ComponentQuery::new()
            .kind(ComponentKind::Container)
            .run(components.iter());

        assert_eq!(results.len(), 2);
    }
}
