//! Result schema and storage.
//!
//! Defines the canonical on-disk record format for benchmark results and
//! provides helpers to serialise and parse them. The format uses a simple
//! key=value text encoding so that no external dependency is required.

use std::collections::HashMap;
use std::time::Duration;

/// A single benchmark result record.
#[derive(Debug, Clone, PartialEq)]
pub struct BenchRecord {
    /// Name of the benchmark.
    pub name: String,
    /// Git commit hash at measurement time (or "unknown").
    pub commit: String,
    /// Minimum observed duration in nanoseconds.
    pub min_ns: u64,
    /// Maximum observed duration in nanoseconds.
    pub max_ns: u64,
    /// Mean duration in nanoseconds.
    pub mean_ns: u64,
    /// Median duration in nanoseconds.
    pub median_ns: u64,
    /// Number of samples collected.
    pub sample_count: u32,
    /// Regression threshold in nanoseconds (`0` means no threshold set).
    pub threshold_ns: u64,
    /// Arbitrary extra fields.
    pub extra: HashMap<String, String>,
}

impl BenchRecord {
    /// Construct a minimal record.
    pub fn new(name: &str, commit: &str) -> Self {
        Self {
            name: name.to_owned(),
            commit: commit.to_owned(),
            min_ns: 0,
            max_ns: 0,
            mean_ns: 0,
            median_ns: 0,
            sample_count: 0,
            threshold_ns: 0,
            extra: HashMap::new(),
        }
    }

    /// Set timing fields from Duration values.
    pub fn with_timings(
        mut self,
        min: Duration,
        max: Duration,
        mean: Duration,
        median: Duration,
        samples: u32,
    ) -> Self {
        self.min_ns = min.as_nanos() as u64;
        self.max_ns = max.as_nanos() as u64;
        self.mean_ns = mean.as_nanos() as u64;
        self.median_ns = median.as_nanos() as u64;
        self.sample_count = samples;
        self
    }

    /// Set the regression threshold.
    pub fn with_threshold(mut self, threshold: Duration) -> Self {
        self.threshold_ns = threshold.as_nanos() as u64;
        self
    }

    /// Returns `true` if a threshold is set and the mean exceeds it.
    pub fn is_regression(&self) -> bool {
        self.threshold_ns > 0 && self.mean_ns > self.threshold_ns
    }
}

/// Serialise a `BenchRecord` to a key=value text format.
pub fn serialize(record: &BenchRecord) -> String {
    let mut lines: Vec<String> = vec![
        format!("name={}", record.name),
        format!("commit={}", record.commit),
        format!("min_ns={}", record.min_ns),
        format!("max_ns={}", record.max_ns),
        format!("mean_ns={}", record.mean_ns),
        format!("median_ns={}", record.median_ns),
        format!("sample_count={}", record.sample_count),
        format!("threshold_ns={}", record.threshold_ns),
    ];
    for (k, v) in &record.extra {
        lines.push(format!("extra.{}={}", k, v));
    }
    lines.join("\n")
}

/// Parse a `BenchRecord` from the key=value text format.
pub fn parse(input: &str) -> Result<BenchRecord, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_owned(), v.trim().to_owned());
        }
    }

    let get = |key: &str| -> Result<String, String> {
        map.get(key)
            .cloned()
            .ok_or_else(|| format!("missing field: {}", key))
    };

    let name = get("name")?;
    let commit = get("commit")?;
    let min_ns: u64 = get("min_ns")?
        .parse()
        .map_err(|e| format!("min_ns: {}", e))?;
    let max_ns: u64 = get("max_ns")?
        .parse()
        .map_err(|e| format!("max_ns: {}", e))?;
    let mean_ns: u64 = get("mean_ns")?
        .parse()
        .map_err(|e| format!("mean_ns: {}", e))?;
    let median_ns: u64 = get("median_ns")?
        .parse()
        .map_err(|e| format!("median_ns: {}", e))?;
    let sample_count: u32 = get("sample_count")?
        .parse()
        .map_err(|e| format!("sample_count: {}", e))?;
    let threshold_ns: u64 = get("threshold_ns")?
        .parse()
        .map_err(|e| format!("threshold_ns: {}", e))?;

    let extra: HashMap<String, String> = map
        .into_iter()
        .filter_map(|(k, v)| k.strip_prefix("extra.").map(|s| (s.to_owned(), v)))
        .collect();

    Ok(BenchRecord {
        name,
        commit,
        min_ns,
        max_ns,
        mean_ns,
        median_ns,
        sample_count,
        threshold_ns,
        extra,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_serialization() {
        let rec = BenchRecord::new("bench-foo", "abc1234")
            .with_timings(
                Duration::from_nanos(100),
                Duration::from_nanos(900),
                Duration::from_nanos(450),
                Duration::from_nanos(420),
                10,
            )
            .with_threshold(Duration::from_micros(1));

        let s = serialize(&rec);
        let parsed = parse(&s).unwrap();
        assert_eq!(parsed, rec);
    }

    #[test]
    fn regression_detected() {
        let rec = BenchRecord::new("slow-bench", "dead0000")
            .with_timings(
                Duration::from_micros(1),
                Duration::from_micros(5),
                Duration::from_micros(3),
                Duration::from_micros(3),
                5,
            )
            .with_threshold(Duration::from_micros(2));
        assert!(rec.is_regression());
    }
}
