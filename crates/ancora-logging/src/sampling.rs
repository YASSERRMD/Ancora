/// Deterministic sampling: include every `rate`-th record.
/// rate=1 means all, rate=10 means 10%, rate=100 means 1%.
pub struct Sampler {
    rate: u64,
    counter: u64,
}

impl Sampler {
    pub fn new(rate: u64) -> Self {
        Self { rate: rate.max(1), counter: 0 }
    }

    pub fn should_sample(&mut self) -> bool {
        self.counter += 1;
        self.counter % self.rate == 0
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
