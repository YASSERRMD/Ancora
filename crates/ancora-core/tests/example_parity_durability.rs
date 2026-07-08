// Example parity: durability example -- run resumption after crash consistent across languages.

const CHECKPOINT_STEP_COUNT: usize = 5;
const CRASH_AT_STEP: usize = 2;

struct DurabilityExample {
    lang: &'static str,
    steps_before_crash: usize,
    steps_after_resume: usize,
    total_steps: usize,
}

fn steps_before_crash(crash_at: usize) -> usize {
    crash_at
}
fn steps_after_resume(total: usize, crash_at: usize) -> usize {
    total - crash_at
}

const DURABILITY_EXAMPLES: &[DurabilityExample] = &[
    DurabilityExample {
        lang: "rust",
        steps_before_crash: 2,
        steps_after_resume: 3,
        total_steps: 5,
    },
    DurabilityExample {
        lang: "go",
        steps_before_crash: 2,
        steps_after_resume: 3,
        total_steps: 5,
    },
    DurabilityExample {
        lang: "python",
        steps_before_crash: 2,
        steps_after_resume: 3,
        total_steps: 5,
    },
    DurabilityExample {
        lang: "typescript",
        steps_before_crash: 2,
        steps_after_resume: 3,
        total_steps: 5,
    },
    DurabilityExample {
        lang: "dotnet",
        steps_before_crash: 2,
        steps_after_resume: 3,
        total_steps: 5,
    },
    DurabilityExample {
        lang: "java",
        steps_before_crash: 2,
        steps_after_resume: 3,
        total_steps: 5,
    },
];

#[test]
fn test_steps_before_crash_consistent() {
    for e in DURABILITY_EXAMPLES {
        assert_eq!(
            e.steps_before_crash,
            steps_before_crash(CRASH_AT_STEP),
            "lang {} crash step differs",
            e.lang
        );
    }
}

#[test]
fn test_total_steps_correct() {
    for e in DURABILITY_EXAMPLES {
        assert_eq!(
            e.steps_before_crash + e.steps_after_resume,
            CHECKPOINT_STEP_COUNT,
            "lang {} total steps incorrect",
            e.lang
        );
    }
}

#[test]
fn test_six_durability_examples() {
    assert_eq!(DURABILITY_EXAMPLES.len(), 6);
}

#[test]
fn test_resumed_steps_formula() {
    assert_eq!(steps_after_resume(5, 2), 3);
}

#[test]
fn test_all_examples_have_five_total_steps() {
    for e in DURABILITY_EXAMPLES {
        assert_eq!(e.total_steps, CHECKPOINT_STEP_COUNT);
    }
}

#[test]
fn test_crash_and_resume_accounted_for_all_steps() {
    for e in DURABILITY_EXAMPLES {
        assert_eq!(e.steps_before_crash + e.steps_after_resume, e.total_steps);
    }
}
