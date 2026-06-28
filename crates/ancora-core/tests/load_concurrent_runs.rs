// Load: simulate N concurrent runs processed sequentially (single-threaded sim).

const CONCURRENT_N: usize = 100;

#[derive(Clone, Debug)]
struct RunSim {
    run_id: String,
    steps: usize,
    completed: usize,
}

impl RunSim {
    fn new(id: usize, steps: usize) -> Self {
        Self { run_id: format!("run-{id:04}"), steps, completed: 0 }
    }
    fn tick(&mut self) { if self.completed < self.steps { self.completed += 1; } }
    fn is_done(&self) -> bool { self.completed >= self.steps }
}

fn run_all(runs: &mut Vec<RunSim>) -> usize {
    loop {
        let pending: Vec<bool> = runs.iter().map(|r| !r.is_done()).collect();
        if !pending.iter().any(|&p| p) { break; }
        for r in runs.iter_mut() { if !r.is_done() { r.tick(); } }
    }
    runs.iter().filter(|r| r.is_done()).count()
}

#[test]
fn test_all_concurrent_runs_complete() {
    let mut runs: Vec<RunSim> = (0..CONCURRENT_N).map(|i| RunSim::new(i, 3)).collect();
    let done = run_all(&mut runs);
    assert_eq!(done, CONCURRENT_N);
}

#[test]
fn test_run_ids_are_unique() {
    let runs: Vec<RunSim> = (0..CONCURRENT_N).map(|i| RunSim::new(i, 1)).collect();
    let mut ids: Vec<&str> = runs.iter().map(|r| r.run_id.as_str()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), CONCURRENT_N);
}

#[test]
fn test_run_with_more_steps_takes_longer() {
    let mut short = vec![RunSim::new(0, 1)];
    let mut long = vec![RunSim::new(1, 10)];
    let mut short_ticks = 0;
    let mut long_ticks = 0;
    loop {
        if short[0].is_done() { break; }
        short[0].tick(); short_ticks += 1;
    }
    loop {
        if long[0].is_done() { break; }
        long[0].tick(); long_ticks += 1;
    }
    assert!(long_ticks > short_ticks);
}

#[test]
fn test_zero_step_run_is_immediately_done() {
    let r = RunSim::new(0, 0);
    assert!(r.is_done());
}

#[test]
fn test_tick_does_not_exceed_steps() {
    let mut r = RunSim::new(0, 3);
    for _ in 0..10 { r.tick(); }
    assert_eq!(r.completed, 3);
}
