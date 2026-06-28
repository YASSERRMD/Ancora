use crate::{DataCategory, DataRecordBuilder, SensitivityLevel};
#[test]
fn builder_defaults_to_internal_generic() {
    let r = DataRecordBuilder::new("r1", "t1", "test").build();
    assert_eq!(r.level, SensitivityLevel::Internal);
    assert_eq!(r.category, DataCategory::Generic);
    assert_eq!(r.created_tick, 0);
}
