// Chaos: process kill and resume from journal checkpoint.

#[derive(Clone, Debug, PartialEq)]
enum StepState {
    Pending,
    Done(String),
}

struct Checkpoint {
    steps: Vec<StepState>,
    killed_at: Option<usize>,
}

impl Checkpoint {
    fn new(n: usize) -> Self {
        Self {
            steps: vec![StepState::Pending; n],
            killed_at: None,
        }
    }

    fn run_step(&mut self, idx: usize, kill_at: Option<usize>) -> bool {
        if kill_at == Some(idx) {
            self.killed_at = Some(idx);
            return false;
        }
        if matches!(self.steps[idx], StepState::Done(_)) {
            return true;
        }
        self.steps[idx] = StepState::Done(format!("result-{idx}"));
        true
    }

    fn resume(&mut self, kill_at: Option<usize>) -> usize {
        let mut completed = 0;
        for i in 0..self.steps.len() {
            if !matches!(self.steps[i], StepState::Done(_)) {
                if !self.run_step(i, kill_at) {
                    break;
                }
            }
            completed += 1;
        }
        completed
    }
}

#[test]
fn test_full_run_completes_all_steps() {
    let mut cp = Checkpoint::new(5);
    let done = cp.resume(None);
    assert_eq!(done, 5);
    for s in &cp.steps {
        assert!(matches!(s, StepState::Done(_)));
    }
}

#[test]
fn test_kill_at_step_two_stops_there() {
    let mut cp = Checkpoint::new(5);
    cp.resume(Some(2));
    assert!(matches!(cp.steps[0], StepState::Done(_)));
    assert!(matches!(cp.steps[1], StepState::Done(_)));
    assert!(matches!(cp.steps[2], StepState::Pending));
}

#[test]
fn test_resume_after_kill_skips_done_steps() {
    let mut cp = Checkpoint::new(5);
    cp.resume(Some(2));
    let done = cp.resume(None);
    assert_eq!(done, 5);
}

#[test]
fn test_already_done_steps_not_re_executed() {
    let mut cp = Checkpoint::new(3);
    cp.steps[0] = StepState::Done("cached".to_string());
    cp.resume(None);
    assert_eq!(cp.steps[0], StepState::Done("cached".to_string()));
}

#[test]
fn test_kill_at_first_step_resumes_from_zero() {
    let mut cp = Checkpoint::new(4);
    cp.resume(Some(0));
    assert_eq!(cp.killed_at, Some(0));
    let done = cp.resume(None);
    assert_eq!(done, 4);
}

#[test]
fn test_consecutive_kills_still_converge() {
    let mut cp = Checkpoint::new(6);
    cp.resume(Some(1));
    cp.resume(Some(3));
    let done = cp.resume(None);
    assert_eq!(done, 6);
}
