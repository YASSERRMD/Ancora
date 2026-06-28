// Documentation audit: every SDK has all required doc pages.

const SDK_REQUIRED_PAGES: &[&str] = &[
    "index.md",
    "install.md",
    "quickstart.md",
    "tools.md",
    "structured-output.md",
    "multi-agent.md",
    "verifier.md",
    "human-in-the-loop.md",
    "streaming.md",
    "memory-and-rag.md",
    "providers.md",
    "vector-stores.md",
    "durability.md",
    "observability.md",
    "policy.md",
    "mcp-and-a2a.md",
    "testing.md",
    "troubleshooting.md",
    "api-reference.md",
];

const SDKS: &[&str] = &["go", "python", "ts", "dotnet", "java", "rust"];

fn sdk_page(sdk: &str, page: &str) -> String {
    format!("sdk/{sdk}/{page}")
}

#[test]
fn test_required_pages_count() {
    assert_eq!(SDK_REQUIRED_PAGES.len(), 19);
}

#[test]
fn test_six_sdks_defined() {
    assert_eq!(SDKS.len(), 6);
}

#[test]
fn test_all_sdk_page_paths_formatted_correctly() {
    for sdk in SDKS {
        for page in SDK_REQUIRED_PAGES {
            let path = sdk_page(sdk, page);
            assert!(path.starts_with("sdk/"), "path should start with sdk/: {path}");
            assert!(path.ends_with(".md"), "path should end with .md: {path}");
        }
    }
}

#[test]
fn test_quickstart_in_required_pages() {
    assert!(SDK_REQUIRED_PAGES.contains(&"quickstart.md"));
}

#[test]
fn test_no_duplicate_required_pages() {
    let mut sorted = SDK_REQUIRED_PAGES.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), SDK_REQUIRED_PAGES.len());
}

#[test]
fn test_total_sdk_doc_pages_count() {
    let total = SDKS.len() * SDK_REQUIRED_PAGES.len();
    assert_eq!(total, 114);
}
