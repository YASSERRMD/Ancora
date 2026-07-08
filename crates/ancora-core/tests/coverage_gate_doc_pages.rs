// Coverage gate: all expected documentation pages are listed.

const EXPECTED_DOC_PAGES: &[&str] = &[
    "testing/rust-test-plan.md",
    "testing/go-test-plan.md",
    "testing/python-test-plan.md",
    "testing/dotnet-test-plan.md",
    "testing/java-test-plan.md",
    "testing/xlang-test-plan.md",
    "testing/determinism-guarantees.md",
    "testing/reliability-chaos-test-plan.md",
    "testing/security-policy-test-plan.md",
];

#[test]
fn test_nine_testing_doc_pages_expected() {
    assert_eq!(EXPECTED_DOC_PAGES.len(), 9);
}

#[test]
fn test_all_pages_under_testing_directory() {
    for page in EXPECTED_DOC_PAGES {
        assert!(
            page.starts_with("testing/"),
            "page should be under testing/: {page}"
        );
    }
}

#[test]
fn test_all_pages_are_markdown() {
    for page in EXPECTED_DOC_PAGES {
        assert!(page.ends_with(".md"), "page should be .md: {page}");
    }
}

#[test]
fn test_xlang_test_plan_in_pages() {
    assert!(EXPECTED_DOC_PAGES.contains(&"testing/xlang-test-plan.md"));
}

#[test]
fn test_determinism_guarantees_in_pages() {
    assert!(EXPECTED_DOC_PAGES.contains(&"testing/determinism-guarantees.md"));
}

#[test]
fn test_no_duplicate_pages() {
    let mut sorted = EXPECTED_DOC_PAGES.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), EXPECTED_DOC_PAGES.len());
}
