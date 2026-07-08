use crate::model::{ModelHandle, ModelMeta, ModelVersion};

fn make_handle(name: &str) -> ModelHandle {
    let v = ModelVersion::next();
    ModelHandle::new(
        ModelMeta {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            memory_bytes: 512,
        },
        v,
    )
}

#[test]
fn test_model_version_monotonic() {
    let v1 = ModelVersion::next();
    let v2 = ModelVersion::next();
    assert!(v2 > v1, "versions must be monotonically increasing");
}

#[test]
fn test_model_handle_pin_and_drop() {
    let h = make_handle("m1");
    assert_eq!(h.pin_count(), 0);
    let pin = h.pin().expect("should pin");
    assert_eq!(h.pin_count(), 1);
    drop(pin);
    assert_eq!(h.pin_count(), 0);
}

#[test]
fn test_model_handle_unload_blocks_new_pins() {
    let h = make_handle("m2");
    h.unload();
    assert!(h.pin().is_none(), "unloaded model must not accept new pins");
}

#[test]
fn test_can_reclaim_only_when_drained() {
    let h = make_handle("m3");
    let pin = h.pin().unwrap();
    h.unload();
    assert!(!h.can_reclaim(), "pin still held, cannot reclaim");
    drop(pin);
    assert!(h.can_reclaim(), "pin released, can reclaim now");
}

#[test]
fn test_multiple_pins_all_must_drop() {
    let h = make_handle("m4");
    let p1 = h.pin().unwrap();
    let p2 = h.pin().unwrap();
    assert_eq!(h.pin_count(), 2);
    drop(p1);
    assert_eq!(h.pin_count(), 1);
    drop(p2);
    assert_eq!(h.pin_count(), 0);
}
