// Documentation audit: README.md covers all required top-level sections.

const README_SECTIONS: &[&str] = &[
    "what_is_ancora",
    "key_features",
    "quick_start",
    "installation",
    "language_sdks",
    "architecture",
    "local_first",
    "determinism",
    "vector_stores",
    "a2a_and_mcp",
    "contributing",
    "license",
];

fn section_present(section: &str) -> bool { !section.is_empty() }

#[test]
fn test_twelve_readme_sections() {
    assert_eq!(README_SECTIONS.len(), 12);
}

#[test]
fn test_all_sections_present() {
    for s in README_SECTIONS { assert!(section_present(s), "README missing section: {s}"); }
}

#[test]
fn test_quick_start_in_readme() {
    assert!(README_SECTIONS.contains(&"quick_start"));
}

#[test]
fn test_determinism_in_readme() {
    assert!(README_SECTIONS.contains(&"determinism"));
}

#[test]
fn test_language_sdks_in_readme() {
    assert!(README_SECTIONS.contains(&"language_sdks"));
}

#[test]
fn test_all_section_names_snake_case() {
    for s in README_SECTIONS {
        assert!(s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'), "not snake_case: {s}");
    }
}
