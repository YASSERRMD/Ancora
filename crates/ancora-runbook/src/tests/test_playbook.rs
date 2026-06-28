use crate::playbook::Playbook;

#[test]
fn add_steps_builds_correctly() {
    let p = Playbook::new("test", "trigger")
        .add_step("step1", "expected1", "fallback1")
        .add_step("step2", "expected2", "fallback2");
    assert_eq!(p.step_count(), 2);
    assert_eq!(p.steps[0].step, 1);
    assert_eq!(p.steps[1].step, 2);
}

#[test]
fn step_fields_are_correct() {
    let p = Playbook::new("p", "t").add_step("do x", "x done", "retry");
    assert_eq!(p.steps[0].action, "do x");
    assert_eq!(p.steps[0].expected_outcome, "x done");
    assert_eq!(p.steps[0].on_failure, "retry");
}
