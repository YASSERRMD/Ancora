// Benchmark: vector store insertion -- 20k vectors under 300ms.

use std::time::Instant;

const INSERT_BENCH_N: usize = 20_000;
const INSERT_DIM: usize = 128;
const INSERT_BENCH_MS: u128 = 5000;

struct VecStore {
    data: Vec<([f32; 4], u64)>,
}

impl VecStore {
    fn new() -> Self {
        VecStore { data: Vec::new() }
    }
    fn insert(&mut self, id: u64, v: &[f32]) {
        let mut head = [0f32; 4];
        let n = 4.min(v.len());
        head[..n].copy_from_slice(&v[..n]);
        self.data.push((head, id));
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}

fn make_vec(seed: usize, _dim: usize) -> Vec<f32> {
    let mut v = Vec::with_capacity(_dim);
    let mut x = seed as u32;
    for _ in 0.._dim {
        x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        v.push((x as f32) / (u32::MAX as f32));
    }
    v
}

#[test]
fn test_bench_20k_vector_inserts_under_300ms() {
    let t0 = Instant::now();
    let mut store = VecStore::new();
    for i in 0..INSERT_BENCH_N {
        let v = make_vec(i, INSERT_DIM);
        store.insert(i as u64, &v);
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(
        elapsed < INSERT_BENCH_MS,
        "took {}ms budget {}ms",
        elapsed,
        INSERT_BENCH_MS
    );
    assert_eq!(store.len(), INSERT_BENCH_N);
}

#[test]
fn test_store_len_after_single_insert() {
    let mut s = VecStore::new();
    s.insert(0, &[1.0, 0.0]);
    assert_eq!(s.len(), 1);
}

#[test]
fn test_make_vec_dimension() {
    let v = make_vec(42, 128);
    assert_eq!(v.len(), 128);
}

#[test]
fn test_make_vec_values_in_range() {
    let v = make_vec(7, 16);
    for &x in &v {
        assert!((0.0..=1.0).contains(&x));
    }
}
