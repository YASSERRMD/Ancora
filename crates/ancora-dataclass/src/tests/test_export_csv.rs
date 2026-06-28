use crate::{DataCategory, DataRecord, SensitivityLevel, to_csv};
#[test]
fn csv_has_header_and_data_rows() {
    let r = DataRecord::new("r1", "t1", "name", SensitivityLevel::Internal, DataCategory::Generic, 10)
        .with_tag("gdpr");
    let csv = to_csv(&[&r]);
    assert!(csv.starts_with("id,tenant_id,name,level,category,tags,created_tick\n"));
    assert!(csv.contains("r1,t1,name,INTERNAL,GENERIC,gdpr,10"));
}
