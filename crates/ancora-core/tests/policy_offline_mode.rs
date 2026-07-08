// Policy: offline mode -- all configured providers must have local-only flag set.

struct ProviderConfig {
    name: &'static str,
    base_url: &'static str,
    local_only: bool,
}

fn validate_offline_mode(providers: &[ProviderConfig]) -> Result<(), Vec<String>> {
    let violations: Vec<String> = providers
        .iter()
        .filter(|p| !p.local_only)
        .map(|p| {
            format!(
                "provider '{}' (url={}) is not local-only",
                p.name, p.base_url
            )
        })
        .collect();
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

const LOCAL_PROVIDERS: &[ProviderConfig] = &[
    ProviderConfig {
        name: "ollama",
        base_url: "http://localhost:11434",
        local_only: true,
    },
    ProviderConfig {
        name: "lmstudio",
        base_url: "http://localhost:1234",
        local_only: true,
    },
];

#[test]
fn test_all_local_providers_pass() {
    assert!(validate_offline_mode(LOCAL_PROVIDERS).is_ok());
}

#[test]
fn test_remote_provider_fails_offline_check() {
    let providers = vec![ProviderConfig {
        name: "openai",
        base_url: "https://api.openai.com",
        local_only: false,
    }];
    assert!(validate_offline_mode(&providers).is_err());
}

#[test]
fn test_mixed_providers_reports_violations() {
    let providers = vec![
        ProviderConfig {
            name: "ollama",
            base_url: "http://localhost:11434",
            local_only: true,
        },
        ProviderConfig {
            name: "openai",
            base_url: "https://api.openai.com",
            local_only: false,
        },
    ];
    let r = validate_offline_mode(&providers);
    let violations = r.unwrap_err();
    assert_eq!(violations.len(), 1);
    assert!(violations[0].contains("openai"));
}

#[test]
fn test_violation_message_includes_url() {
    let providers = vec![ProviderConfig {
        name: "anthropic",
        base_url: "https://api.anthropic.com",
        local_only: false,
    }];
    let violations = validate_offline_mode(&providers).unwrap_err();
    assert!(violations[0].contains("api.anthropic.com"));
}

#[test]
fn test_empty_providers_passes_offline_check() {
    assert!(validate_offline_mode(&[]).is_ok());
}

#[test]
fn test_all_remote_all_reported() {
    let providers = vec![
        ProviderConfig {
            name: "a",
            base_url: "https://a.example",
            local_only: false,
        },
        ProviderConfig {
            name: "b",
            base_url: "https://b.example",
            local_only: false,
        },
    ];
    let violations = validate_offline_mode(&providers).unwrap_err();
    assert_eq!(violations.len(), 2);
}
