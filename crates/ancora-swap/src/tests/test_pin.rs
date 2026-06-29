use crate::pin::PinRegistry;
use crate::runtime::{make_model, RunId};

#[test]
fn test_pin_registry_basic() {
    let reg = PinRegistry::new();
    let run = RunId(1);
    let model = make_model("pinned");
    let ver = model.version();

    reg.pin_run(run, model);
    let got = reg.get(run).expect("model must be in registry");
    assert_eq!(got.version(), ver);
}

#[test]
fn test_pin_registry_unpin() {
    let reg = PinRegistry::new();
    let run = RunId(2);
    reg.pin_run(run, make_model("x"));
    assert_eq!(reg.len(), 1);
    let removed = reg.unpin_run(run);
    assert!(removed.is_some());
    assert_eq!(reg.len(), 0);
}

#[test]
fn test_pin_registry_overwrite() {
    let reg = PinRegistry::new();
    let run = RunId(3);
    let m1 = make_model("first");
    let m2 = make_model("second");
    let v2 = m2.version();

    reg.pin_run(run, m1);
    reg.pin_run(run, m2);
    // Second pin overwrites first.
    assert_eq!(reg.get(run).unwrap().version(), v2);
}
