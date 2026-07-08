use crate::key::{HsmKey, HsmKeyAlgorithm};

pub struct HsmPolicy {
    pub allow_extractable: bool,
    pub min_key_bits: Option<u32>,
    pub blocked_algorithms: Vec<HsmKeyAlgorithm>,
}

impl HsmPolicy {
    pub fn new() -> Self {
        Self {
            allow_extractable: false,
            min_key_bits: None,
            blocked_algorithms: Vec::new(),
        }
    }
    pub fn allow_extractable(mut self) -> Self {
        self.allow_extractable = true;
        self
    }
    pub fn min_key_bits(mut self, bits: u32) -> Self {
        self.min_key_bits = Some(bits);
        self
    }
    pub fn block_algorithm(mut self, algo: HsmKeyAlgorithm) -> Self {
        self.blocked_algorithms.push(algo);
        self
    }

    pub fn is_allowed(&self, key: &HsmKey) -> bool {
        if !self.allow_extractable && key.extractable {
            return false;
        }
        if self.blocked_algorithms.contains(&key.algorithm) {
            return false;
        }
        true
    }

    pub fn algorithm_allowed(&self, algo: &HsmKeyAlgorithm) -> bool {
        !self.blocked_algorithms.contains(algo)
    }
}
