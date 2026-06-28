/// A sliding-window counter keyed by (start_tick, window_secs).
/// Ticks are abstract monotonic u64 units (seconds in practice).
#[derive(Debug, Default)]
pub struct SlidingWindow {
    pub window_secs: u64,
    pub window_start: u64,
    pub count: u64,
}

impl SlidingWindow {
    pub fn new(window_secs: u64, now: u64) -> Self {
        Self { window_secs, window_start: now, count: 0 }
    }

    /// Advance to `now`, resetting the counter if the window has elapsed.
    pub fn tick(&mut self, now: u64) {
        if now >= self.window_start + self.window_secs {
            self.count = 0;
            self.window_start = now;
        }
    }

    pub fn increment(&mut self, now: u64, by: u64) {
        self.tick(now);
        self.count += by;
    }

    pub fn value(&mut self, now: u64) -> u64 {
        self.tick(now);
        self.count
    }

    /// Seconds until the current window resets.
    pub fn seconds_until_reset(&self, now: u64) -> u64 {
        let end = self.window_start + self.window_secs;
        if now >= end { 0 } else { end - now }
    }
}
