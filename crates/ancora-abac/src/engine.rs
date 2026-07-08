use crate::attribute::AttributeSet;
use crate::policy::{Effect, PolicyStore};

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny(String),
    NotApplicable,
}

pub struct PolicyEngine<'a> {
    store: &'a PolicyStore,
}

impl<'a> PolicyEngine<'a> {
    pub fn new(store: &'a PolicyStore) -> Self {
        Self { store }
    }

    pub fn evaluate(
        &self,
        action: &str,
        subject: &AttributeSet,
        resource: &AttributeSet,
        env: &AttributeSet,
    ) -> Decision {
        let mut last_deny: Option<String> = None;
        for policy in self.store.policies() {
            match policy.evaluate(action, subject, resource, env) {
                Some(Effect::Deny) => {
                    last_deny = Some(format!("denied by policy {}", policy.id));
                }
                Some(Effect::Allow) if last_deny.is_none() => {
                    return Decision::Allow;
                }
                _ => {}
            }
        }
        if let Some(reason) = last_deny {
            Decision::Deny(reason)
        } else {
            Decision::NotApplicable
        }
    }

    pub fn is_allowed(
        &self,
        action: &str,
        subject: &AttributeSet,
        resource: &AttributeSet,
        env: &AttributeSet,
    ) -> bool {
        self.evaluate(action, subject, resource, env) == Decision::Allow
    }
}
