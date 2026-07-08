use crate::catalog_e2e::{CatalogTool, ToolCatalog};

#[test]
fn test_catalog_install_adds_tool() {
    let mut catalog = ToolCatalog::new();
    let tool = CatalogTool::new("search-tool", "Performs semantic search", 1);
    catalog.install(tool).expect("install must succeed");
    assert_eq!(catalog.count(), 1);
    let found = catalog
        .get("search-tool")
        .expect("tool must be retrievable");
    assert_eq!(found.plugin_id, 1);
}

#[test]
fn test_catalog_duplicate_install_fails() {
    let mut catalog = ToolCatalog::new();
    catalog
        .install(CatalogTool::new("dup-tool", "desc", 2))
        .unwrap();
    let result = catalog.install(CatalogTool::new("dup-tool", "desc2", 3));
    assert!(result.is_err());
}

#[test]
fn test_catalog_remove_tool() {
    let mut catalog = ToolCatalog::new();
    catalog
        .install(CatalogTool::new("rm-tool", "removable", 4))
        .unwrap();
    assert!(catalog.remove("rm-tool"));
    assert!(catalog.get("rm-tool").is_none());
    assert_eq!(catalog.count(), 0);
}

#[test]
fn test_catalog_list_sorted() {
    let mut catalog = ToolCatalog::new();
    catalog
        .install(CatalogTool::new("z-tool", "last", 5))
        .unwrap();
    catalog
        .install(CatalogTool::new("a-tool", "first", 6))
        .unwrap();
    let list = catalog.list();
    assert_eq!(list[0].name, "a-tool");
    assert_eq!(list[1].name, "z-tool");
}
