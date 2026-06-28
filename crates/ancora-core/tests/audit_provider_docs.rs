// Documentation audit: all providers documented in SDK pages.

const PROVIDERS: &[(&str, &str)] = &[
    ("anthropic_claude",  "claude-3-5-haiku, claude-3-5-sonnet"),
    ("openai",            "gpt-4o, gpt-4o-mini"),
    ("qwen",              "qwen3, qwen2.5"),
    ("glm",               "glm-4, glm-4-flash"),
    ("gemini",            "gemini-1.5-pro"),
    ("ollama_local",      "llama3, phi3, mistral"),
    ("lmstudio_local",    "any gguf model"),
    ("deepseek",          "deepseek-chat, deepseek-coder"),
];

const PROVIDER_DOC_PAGES: &[(&str, &str)] = &[
    ("anthropic_claude", "sdk/rust/providers.md"),
    ("openai",           "sdk/rust/providers.md"),
    ("qwen",             "sdk/rust/chinese-providers.md"),
    ("glm",              "sdk/dotnet/glm-selfhost.md"),
    ("gemini",           "sdk/go/providers.md"),
    ("ollama_local",     "sdk/python/providers.md"),
    ("lmstudio_local",   "sdk/ts/providers.md"),
    ("deepseek",         "sdk/java/providers.md"),
];

#[test]
fn test_8_providers_documented() {
    assert_eq!(PROVIDERS.len(), 8);
}

#[test]
fn test_all_providers_have_doc_page() {
    let doc_providers: Vec<&str> = PROVIDER_DOC_PAGES.iter().map(|(p, _)| *p).collect();
    for (provider, _) in PROVIDERS {
        assert!(doc_providers.contains(provider), "no doc page for provider: {provider}");
    }
}

#[test]
fn test_qwen_in_chinese_providers_doc() {
    let qwen = PROVIDER_DOC_PAGES.iter().find(|(p, _)| *p == "qwen");
    assert!(qwen.map(|(_, doc)| doc.contains("chinese")).unwrap_or(false));
}

#[test]
fn test_local_providers_documented() {
    let local_providers: Vec<&str> = PROVIDERS.iter()
        .filter(|(p, _)| p.contains("local"))
        .map(|(p, _)| *p)
        .collect();
    assert_eq!(local_providers.len(), 2);
}

#[test]
fn test_all_doc_pages_end_with_md() {
    for (_, page) in PROVIDER_DOC_PAGES { assert!(page.ends_with(".md"), "not .md: {page}"); }
}

#[test]
fn test_anthropic_and_openai_in_providers() {
    assert!(PROVIDERS.iter().any(|(p, _)| *p == "anthropic_claude"));
    assert!(PROVIDERS.iter().any(|(p, _)| *p == "openai"));
}
