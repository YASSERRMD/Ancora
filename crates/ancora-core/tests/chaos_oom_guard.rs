// Chaos: OOM guard -- enforce memory budget on buffered output.

struct MemBudget {
    used: usize,
    limit: usize,
}

impl MemBudget {
    fn new(limit: usize) -> Self {
        Self { used: 0, limit }
    }
    fn try_alloc(&mut self, bytes: usize) -> Result<(), String> {
        if self.used + bytes > self.limit {
            Err(format!(
                "oom: need {} have {}",
                bytes,
                self.limit - self.used
            ))
        } else {
            self.used += bytes;
            Ok(())
        }
    }
    fn free(&mut self, bytes: usize) {
        self.used = self.used.saturating_sub(bytes);
    }
    fn used_pct(&self) -> f64 {
        self.used as f64 / self.limit as f64 * 100.0
    }
}

#[test]
fn test_alloc_within_limit_succeeds() {
    let mut b = MemBudget::new(1024);
    assert!(b.try_alloc(512).is_ok());
    assert_eq!(b.used, 512);
}

#[test]
fn test_alloc_exceeds_limit_fails() {
    let mut b = MemBudget::new(100);
    let r = b.try_alloc(200);
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("oom"));
}

#[test]
fn test_free_restores_capacity() {
    let mut b = MemBudget::new(100);
    b.try_alloc(80).unwrap();
    b.free(80);
    assert!(b.try_alloc(100).is_ok());
}

#[test]
fn test_sequential_allocs_accumulate() {
    let mut b = MemBudget::new(300);
    b.try_alloc(100).unwrap();
    b.try_alloc(100).unwrap();
    let r = b.try_alloc(101);
    assert!(r.is_err());
}

#[test]
fn test_used_pct_correct() {
    let mut b = MemBudget::new(200);
    b.try_alloc(100).unwrap();
    let pct = b.used_pct();
    assert!((pct - 50.0).abs() < 0.001);
}

#[test]
fn test_exact_limit_alloc_succeeds() {
    let mut b = MemBudget::new(64);
    assert!(b.try_alloc(64).is_ok());
}
