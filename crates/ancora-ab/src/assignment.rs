/// Assigns subjects to variants using a deterministic hash of a key.
use crate::experiment::Experiment;

/// Result of a variant assignment.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub subject_key: String,
    pub experiment_id: String,
    pub variant_name: String,
}

/// Assign a subject to a variant deterministically.
///
/// The assignment is stable: the same (experiment_id, subject_key) pair
/// always produces the same variant, with no external state.
pub fn assign(experiment: &Experiment, subject_key: &str) -> Assignment {
    let bucket = deterministic_bucket(subject_key, &experiment.id);
    let variant_name = bucket_to_variant(experiment, bucket);
    Assignment {
        subject_key: subject_key.to_string(),
        experiment_id: experiment.id.clone(),
        variant_name,
    }
}

/// Map a [0,1) bucket value to a variant using cumulative weights.
fn bucket_to_variant(experiment: &Experiment, bucket: f64) -> String {
    let mut cumulative = 0.0;
    for variant in &experiment.variants {
        cumulative += variant.traffic_weight;
        if bucket < cumulative {
            return variant.name.clone();
        }
    }
    // Fallback to the last variant (handles floating-point edge cases).
    experiment.variants.last().unwrap().name.clone()
}

/// Produce a deterministic [0,1) value from the experiment id and subject key.
///
/// Uses a simple FNV-1a-inspired hash; no external crates required.
pub fn deterministic_bucket(subject_key: &str, experiment_id: &str) -> f64 {
    let combined = format!("{experiment_id}:{subject_key}");
    let hash = fnv1a_hash(combined.as_bytes());
    // Map to [0, 1).
    (hash as f64) / (u64::MAX as f64)
}

fn fnv1a_hash(data: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;
    let mut hash = FNV_OFFSET;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    // Mix the bits further with xorshift to improve distribution uniformity.
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xff51afd7ed558ccd);
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
    hash ^= hash >> 33;
    hash
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn bucket_is_in_range() {
        let b = deterministic_bucket("user-42", "exp-1");
        assert!((0.0..1.0).contains(&b));
    }

    #[test]
    fn same_input_same_bucket() {
        let b1 = deterministic_bucket("alice", "exp-xyz");
        let b2 = deterministic_bucket("alice", "exp-xyz");
        assert_eq!(b1, b2);
    }

    #[test]
    fn different_input_different_bucket() {
        let b1 = deterministic_bucket("alice", "exp-xyz");
        let b2 = deterministic_bucket("bob", "exp-xyz");
        assert_ne!(b1, b2);
    }
}
