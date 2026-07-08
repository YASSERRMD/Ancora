use crate::policy::{Effect, PolicyStore};

pub struct PolicyReport {
    pub total: usize,
    pub allow_count: usize,
    pub deny_count: usize,
    pub wildcard_count: usize,
}

pub fn report(store: &PolicyStore) -> PolicyReport {
    let mut allow_count = 0;
    let mut deny_count = 0;
    let mut wildcard_count = 0;
    for p in store.policies() {
        match p.effect {
            Effect::Allow => allow_count += 1,
            Effect::Deny => deny_count += 1,
        }
        if p.actions.iter().any(|a| a == "*") {
            wildcard_count += 1;
        }
    }
    PolicyReport {
        total: store.count(),
        allow_count,
        deny_count,
        wildcard_count,
    }
}
