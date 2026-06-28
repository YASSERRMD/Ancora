// Reliability: per-run deadline enforcement (simulated, no real sleep).

struct Deadline {
    start_ns: u64,
    limit_ns: u64,
}

impl Deadline {
    fn new(start_ns: u64, limit_ns: u64) -> Self { Self { start_ns, limit_ns } }
    fn exceeded(&self, now_ns: u64) -> bool { now_ns.saturating_sub(self.start_ns) > self.limit_ns }
    fn remaining_ns(&self, now_ns: u64) -> u64 {
        let elapsed = now_ns.saturating_sub(self.start_ns);
        self.limit_ns.saturating_sub(elapsed)
    }
}

fn run_until_deadline(steps: &[u64], deadline: &Deadline) -> (usize, bool) {
    let mut completed = 0;
    for &step_end_ns in steps {
        if deadline.exceeded(step_end_ns) {
            return (completed, false);
        }
        completed += 1;
    }
    (completed, true)
}

#[test]
fn test_run_completes_within_deadline() {
    let d = Deadline::new(0, 1_000_000_000);
    let steps = vec![100_000_000u64, 200_000_000, 300_000_000];
    let (n, ok) = run_until_deadline(&steps, &d);
    assert!(ok);
    assert_eq!(n, 3);
}

#[test]
fn test_run_aborts_past_deadline() {
    let d = Deadline::new(0, 100_000_000);
    let steps = vec![50_000_000u64, 150_000_000, 200_000_000];
    let (n, ok) = run_until_deadline(&steps, &d);
    assert!(!ok);
    assert_eq!(n, 1);
}

#[test]
fn test_exceeded_returns_true_after_limit() {
    let d = Deadline::new(0, 1000);
    assert!(d.exceeded(1001));
    assert!(!d.exceeded(999));
    assert!(!d.exceeded(1000));
}

#[test]
fn test_remaining_decreases_over_time() {
    let d = Deadline::new(0, 1000);
    let r0 = d.remaining_ns(0);
    let r1 = d.remaining_ns(500);
    assert!(r0 > r1);
}

#[test]
fn test_remaining_zero_when_past_deadline() {
    let d = Deadline::new(0, 100);
    assert_eq!(d.remaining_ns(200), 0);
}

#[test]
fn test_no_steps_always_succeeds() {
    let d = Deadline::new(0, 0);
    let (n, ok) = run_until_deadline(&[], &d);
    assert!(ok);
    assert_eq!(n, 0);
}
