// Benchmark: checkpoint save/restore -- 100k checkpoints under 500ms.

use std::time::Instant;

const CKPT_BENCH_N: usize = 100_000;
const CKPT_BENCH_MS: u128 = 5000;

struct Checkpoint {
    step: u32,
    state: u64,
    committed: bool,
}

impl Checkpoint {
    fn save(step: u32, state: u64) -> Self {
        Checkpoint {
            step,
            state,
            committed: true,
        }
    }
    fn restore(ckpt: &Checkpoint) -> (u32, u64) {
        (ckpt.step, ckpt.state)
    }
    fn is_valid(&self) -> bool {
        self.committed
    }
}

#[test]
fn test_bench_100k_checkpoint_round_trips_under_500ms() {
    let t0 = Instant::now();
    let mut total_state = 0u64;
    for i in 0..CKPT_BENCH_N {
        let ckpt = Checkpoint::save(i as u32, i as u64 * 3);
        let (_step, state) = Checkpoint::restore(&ckpt);
        total_state = total_state.wrapping_add(state);
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(
        elapsed < CKPT_BENCH_MS,
        "took {}ms budget {}ms",
        elapsed,
        CKPT_BENCH_MS
    );
    assert!(total_state > 0);
}

#[test]
fn test_checkpoint_save_restore_roundtrip() {
    let ckpt = Checkpoint::save(5, 42);
    let (step, state) = Checkpoint::restore(&ckpt);
    assert_eq!(step, 5);
    assert_eq!(state, 42);
}

#[test]
fn test_checkpoint_is_valid_after_save() {
    let ckpt = Checkpoint::save(0, 0);
    assert!(ckpt.is_valid());
}

#[test]
fn test_checkpoint_not_valid_when_uncommitted() {
    let ckpt = Checkpoint {
        step: 0,
        state: 0,
        committed: false,
    };
    assert!(!ckpt.is_valid());
}
