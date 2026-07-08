use crate::record::DataRecord;

pub fn to_csv(records: &[&DataRecord]) -> String {
    let mut out = String::from("id,tenant_id,name,level,category,tags,created_tick\n");
    for r in records {
        out.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            r.id,
            r.tenant_id,
            r.name,
            r.level,
            r.category,
            r.tags.join("|"),
            r.created_tick,
        ));
    }
    out
}

pub fn to_json(records: &[&DataRecord]) -> String {
    let mut out = String::from("[");
    for (i, r) in records.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            r#"{{"id":"{0}","tenant_id":"{1}","name":"{2}","level":"{3}","category":"{4}","created_tick":{5}}}"#,
            r.id, r.tenant_id, r.name, r.level, r.category, r.created_tick,
        ));
    }
    out.push(']');
    out
}
