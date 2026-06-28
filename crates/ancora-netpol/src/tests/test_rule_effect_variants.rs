use crate::Effect;
#[test]
fn effect_variants_are_distinct() {
    assert_ne!(Effect::Allow, Effect::Deny);
}
