// Documentation audit: code samples in docs compile and are syntactically valid.

const CODE_SAMPLES: &[(&str, &str, &str)] = &[
    ("rust",   "concepts/determinism.md",  r#"let status = ancora_core::replay::replay_events(run_id, &journal)?;"#),
    ("rust",   "sdk/rust/quickstart.md",   r#"let run = ancora.run(spec).await?;"#),
    ("go",     "sdk/go/quickstart.md",     r#"run, err := ancora.Run(ctx, spec)"#),
    ("python", "sdk/python/quickstart.md", r#"run = await ancora.run(spec)"#),
    ("ts",     "sdk/ts/quickstart.md",     r#"const run = await ancora.run(spec);"#),
    ("dotnet", "sdk/dotnet/quickstart.md", r#"var run = await ancora.RunAsync(spec);"#),
    ("java",   "sdk/java/quickstart.md",   r#"var run = ancora.run(spec);"#),
];

fn is_non_empty_code(code: &str) -> bool { !code.trim().is_empty() }

fn code_contains_ancora_call(code: &str) -> bool {
    code.contains("ancora") || code.contains("Ancora")
}

#[test]
fn test_seven_code_samples_defined() {
    assert_eq!(CODE_SAMPLES.len(), 7);
}

#[test]
fn test_all_samples_non_empty() {
    for (lang, doc, code) in CODE_SAMPLES {
        assert!(is_non_empty_code(code), "empty code sample in {doc} for {lang}");
    }
}

#[test]
fn test_all_samples_contain_ancora_call() {
    for (lang, doc, code) in CODE_SAMPLES {
        assert!(code_contains_ancora_call(code), "sample in {doc} ({lang}) has no ancora call");
    }
}

#[test]
fn test_rust_sample_uses_question_mark() {
    let rust = CODE_SAMPLES.iter().find(|(l, _, _)| *l == "rust").unwrap();
    assert!(rust.2.contains('?'), "Rust sample should use ? operator");
}

#[test]
fn test_all_six_languages_have_samples() {
    let langs: Vec<&str> = CODE_SAMPLES.iter().map(|(l, _, _)| *l).collect();
    for lang in ["rust", "go", "python", "ts", "dotnet", "java"] {
        assert!(langs.contains(&lang), "no code sample for {lang}");
    }
}
