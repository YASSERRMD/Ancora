//! Tool-grounded fact checking: ground claims against an external tool function.

#[derive(Debug, Clone)]
pub struct FactCheck {
    pub claim: String,
    pub grounded: bool,
    pub source: String,
}

pub struct FactChecker;

impl FactChecker {
    pub fn check<F: FnMut(&str) -> Option<String>>(claim: &str, mut tool_fn: F) -> FactCheck {
        match tool_fn(claim) {
            Some(source) => FactCheck {
                claim: claim.to_string(),
                grounded: true,
                source,
            },
            None => FactCheck {
                claim: claim.to_string(),
                grounded: false,
                source: String::new(),
            },
        }
    }
}
