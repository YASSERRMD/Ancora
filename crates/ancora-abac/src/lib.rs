pub mod attribute;
pub mod condition;
pub mod context;
pub mod engine;
pub mod policy;

pub use attribute::{AttributeSet, AttributeValue};
pub use condition::Condition;
pub use context::RequestContext;
pub use engine::{Decision, PolicyEngine};
pub use policy::{Effect, Policy, PolicyStore};

#[cfg(test)]
mod tests {
    mod test_attribute_set;
    mod test_attribute_types;
    mod test_condition_eq;
    mod test_condition_gt_lt;
    mod test_condition_contains;
    mod test_condition_and_or;
    mod test_condition_not;
    mod test_condition_exists;
    mod test_policy_action_match;
    mod test_policy_priority;
    mod test_engine_allow;
    mod test_engine_deny;
    mod test_engine_not_applicable;
    mod test_engine_deny_wins;
    mod test_request_context;
    mod test_multi_policy;
    mod test_attribute_from;
    mod test_complex_condition;
    mod test_wildcard_action;
    mod test_env_attributes;
}
