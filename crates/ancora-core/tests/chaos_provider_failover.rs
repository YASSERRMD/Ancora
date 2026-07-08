// Chaos: provider failover -- primary fails, secondary serves request.

#[derive(Debug, PartialEq)]
enum ProviderState {
    Healthy,
    Down,
}

struct Provider {
    name: &'static str,
    state: ProviderState,
}

impl Provider {
    fn call(&self, prompt: &str) -> Result<String, String> {
        match self.state {
            ProviderState::Healthy => Ok(format!("{}: response to '{}'", self.name, prompt)),
            ProviderState::Down => Err(format!("{} is down", self.name)),
        }
    }
}

fn call_with_failover(providers: &[Provider], prompt: &str) -> Result<String, String> {
    for p in providers {
        if let Ok(resp) = p.call(prompt) {
            return Ok(resp);
        }
    }
    Err("all providers failed".to_string())
}

#[test]
fn test_primary_serves_when_healthy() {
    let providers = vec![
        Provider {
            name: "primary",
            state: ProviderState::Healthy,
        },
        Provider {
            name: "secondary",
            state: ProviderState::Healthy,
        },
    ];
    let r = call_with_failover(&providers, "hello");
    assert!(r.unwrap().starts_with("primary:"));
}

#[test]
fn test_secondary_serves_when_primary_down() {
    let providers = vec![
        Provider {
            name: "primary",
            state: ProviderState::Down,
        },
        Provider {
            name: "secondary",
            state: ProviderState::Healthy,
        },
    ];
    let r = call_with_failover(&providers, "hello");
    assert!(r.unwrap().starts_with("secondary:"));
}

#[test]
fn test_all_down_returns_error() {
    let providers = vec![
        Provider {
            name: "primary",
            state: ProviderState::Down,
        },
        Provider {
            name: "secondary",
            state: ProviderState::Down,
        },
    ];
    let r = call_with_failover(&providers, "hello");
    assert!(r.is_err());
    assert_eq!(r.unwrap_err(), "all providers failed");
}

#[test]
fn test_tertiary_serves_when_first_two_down() {
    let providers = vec![
        Provider {
            name: "p1",
            state: ProviderState::Down,
        },
        Provider {
            name: "p2",
            state: ProviderState::Down,
        },
        Provider {
            name: "p3",
            state: ProviderState::Healthy,
        },
    ];
    let r = call_with_failover(&providers, "q");
    assert!(r.unwrap().starts_with("p3:"));
}

#[test]
fn test_response_contains_prompt() {
    let providers = vec![Provider {
        name: "x",
        state: ProviderState::Healthy,
    }];
    let r = call_with_failover(&providers, "my-prompt");
    assert!(r.unwrap().contains("my-prompt"));
}

#[test]
fn test_empty_provider_list_returns_error() {
    let providers: Vec<Provider> = vec![];
    let r = call_with_failover(&providers, "q");
    assert!(r.is_err());
}
