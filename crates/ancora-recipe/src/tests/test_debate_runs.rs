use crate::debate::{build, count_by_stance, Argument, Stance};
use crate::params::ParamSet;

#[test]
fn debate_recipe_builds() {
    let mut ps = ParamSet::new();
    ps.set("rounds", "3");
    ps.set("agents", "2");
    let r = build(&ps);
    assert!(r.validate().is_ok());
    assert_eq!(r.id, "multi-agent-debate");
    // setup + 3 rounds + verdict = 5
    assert_eq!(r.step_count(), 5);
}

#[test]
fn debate_default_params() {
    let ps = ParamSet::default();
    let r = build(&ps);
    // setup + 2 rounds + verdict = 4
    assert_eq!(r.step_count(), 4);
}

#[test]
fn stance_counts_correct() {
    let args = vec![
        Argument::new(0, 1, Stance::For, "a"),
        Argument::new(1, 1, Stance::For, "b"),
        Argument::new(2, 1, Stance::Against, "c"),
        Argument::new(3, 1, Stance::Neutral, "d"),
    ];
    let (f, a, n) = count_by_stance(&args);
    assert_eq!(f, 2);
    assert_eq!(a, 1);
    assert_eq!(n, 1);
}

#[test]
fn empty_args_zero_counts() {
    let (f, a, n) = count_by_stance(&[]);
    assert_eq!((f, a, n), (0, 0, 0));
}
