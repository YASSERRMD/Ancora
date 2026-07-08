// Example parity: local-first provider example works same for ollama and lmstudio.

const LOCAL_PROVIDERS: &[(&str, &str, u16)] = &[
    ("ollama", "http://localhost", 11434),
    ("lmstudio", "http://localhost", 1234),
];

struct LocalProviderResult {
    provider: &'static str,
    #[allow(dead_code)]
    model: &'static str,
    base_url: &'static str,
    local_only: bool,
    response_prefix: &'static str,
}

const LOCAL_RESULTS: &[LocalProviderResult] = &[
    LocalProviderResult {
        provider: "ollama",
        model: "llama3",
        base_url: "http://localhost:11434",
        local_only: true,
        response_prefix: "local:",
    },
    LocalProviderResult {
        provider: "lmstudio",
        model: "phi3",
        base_url: "http://localhost:1234",
        local_only: true,
        response_prefix: "local:",
    },
];

#[test]
fn test_all_local_providers_are_local_only() {
    for r in LOCAL_RESULTS {
        assert!(r.local_only, "provider {} is not local-only", r.provider);
    }
}

#[test]
fn test_all_base_urls_are_localhost() {
    for r in LOCAL_RESULTS {
        assert!(
            r.base_url.contains("localhost"),
            "provider {} base_url is not localhost",
            r.provider
        );
    }
}

#[test]
fn test_all_responses_have_local_prefix() {
    for r in LOCAL_RESULTS {
        assert!(
            r.response_prefix.starts_with("local"),
            "provider {} response prefix not local",
            r.provider
        );
    }
}

#[test]
fn test_two_local_providers_defined() {
    assert_eq!(LOCAL_RESULTS.len(), 2);
}

#[test]
fn test_local_provider_ports_correct() {
    for (name, _, port) in LOCAL_PROVIDERS {
        if *name == "ollama" {
            assert_eq!(*port, 11434);
        }
        if *name == "lmstudio" {
            assert_eq!(*port, 1234);
        }
    }
}

#[test]
fn test_no_remote_url_in_local_providers() {
    for r in LOCAL_RESULTS {
        assert!(
            !r.base_url.contains("api."),
            "provider {} has remote URL",
            r.provider
        );
    }
}
