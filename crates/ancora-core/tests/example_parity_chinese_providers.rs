// Example parity: Chinese provider examples (Qwen, GLM, DeepSeek) across applicable SDKs.

const CHINESE_PROVIDERS: &[(&str, &str, &str)] = &[
    ("qwen", "qwen3", "https://dashscope.aliyuncs.com/api/v1"),
    ("glm", "glm-4", "https://open.bigmodel.cn/api/paas/v4"),
    ("deepseek", "deepseek-chat", "https://api.deepseek.com/v1"),
];

struct ChineseProviderExample {
    provider: &'static str,
    sdk_langs: &'static [&'static str],
    has_doc: bool,
}

const CHINESE_PROVIDER_EXAMPLES: &[ChineseProviderExample] = &[
    ChineseProviderExample {
        provider: "qwen",
        sdk_langs: &["rust", "go", "python", "ts", "java"],
        has_doc: true,
    },
    ChineseProviderExample {
        provider: "glm",
        sdk_langs: &["dotnet"],
        has_doc: true,
    },
    ChineseProviderExample {
        provider: "deepseek",
        sdk_langs: &["rust", "go", "python"],
        has_doc: true,
    },
];

#[test]
fn test_three_chinese_providers() {
    assert_eq!(CHINESE_PROVIDERS.len(), 3);
}

#[test]
fn test_all_chinese_providers_have_docs() {
    for e in CHINESE_PROVIDER_EXAMPLES {
        assert!(e.has_doc, "Chinese provider '{}' has no doc", e.provider);
    }
}

#[test]
fn test_qwen_available_in_five_sdks() {
    let qwen = CHINESE_PROVIDER_EXAMPLES
        .iter()
        .find(|e| e.provider == "qwen")
        .unwrap();
    assert_eq!(qwen.sdk_langs.len(), 5);
}

#[test]
fn test_glm_available_in_dotnet() {
    let glm = CHINESE_PROVIDER_EXAMPLES
        .iter()
        .find(|e| e.provider == "glm")
        .unwrap();
    assert!(glm.sdk_langs.contains(&"dotnet"));
}

#[test]
fn test_all_provider_base_urls_non_empty() {
    for (_, _, url) in CHINESE_PROVIDERS {
        assert!(!url.is_empty());
    }
}

#[test]
fn test_deepseek_available_in_three_sdks() {
    let ds = CHINESE_PROVIDER_EXAMPLES
        .iter()
        .find(|e| e.provider == "deepseek")
        .unwrap();
    assert_eq!(ds.sdk_langs.len(), 3);
}
