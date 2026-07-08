use crate::quickstarts::{for_language, Language, Quickstart, QuickstartStep};

#[test]
fn all_language_quickstarts_have_steps() {
    let guides = vec![
        Quickstart::new(Language::Rust, "Observability")
            .add_step(QuickstartStep::new("Install", "Add crate.")),
        Quickstart::new(Language::Python, "Observability")
            .add_step(QuickstartStep::new("Install", "pip install ancora-obs.")),
        Quickstart::new(Language::Go, "Observability")
            .add_step(QuickstartStep::new("Install", "go get ancora-obs.")),
        Quickstart::new(Language::TypeScript, "Observability")
            .add_step(QuickstartStep::new("Install", "npm install @ancora/obs.")),
    ];

    for guide in &guides {
        assert!(
            guide.step_count() >= 1,
            "{} quickstart has no steps",
            guide.language
        );
    }
}

#[test]
fn filter_quickstarts_by_language_returns_correct_count() {
    let guides = vec![
        Quickstart::new(Language::Rust, "obs").add_step(QuickstartStep::new("s", "d")),
        Quickstart::new(Language::Rust, "eval").add_step(QuickstartStep::new("s", "d")),
        Quickstart::new(Language::Go, "obs").add_step(QuickstartStep::new("s", "d")),
    ];
    let rust_guides = for_language(&guides, &Language::Rust);
    assert_eq!(rust_guides.len(), 2);
}

#[test]
fn quickstart_render_produces_content() {
    let qs = Quickstart::new(Language::Rust, "Test")
        .add_step(QuickstartStep::new("Step 1", "Description").with_command("cargo test"));
    let rendered = qs.render();
    assert!(rendered.contains("cargo test"));
    assert!(rendered.contains("Rust Quickstart"));
}
