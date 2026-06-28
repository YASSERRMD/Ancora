/// Parity check between languages/SDKs for a given feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParityState {
    Full,
    Partial { missing: Vec<String> },
    Missing,
}

#[derive(Debug, Clone)]
pub struct ParityEntry {
    pub feature: String,
    pub state: ParityState,
}

impl ParityEntry {
    pub fn new(feature: impl Into<String>, state: ParityState) -> Self {
        Self {
            feature: feature.into(),
            state,
        }
    }

    pub fn is_full_parity(&self) -> bool {
        self.state == ParityState::Full
    }

    pub fn describe(&self) -> String {
        match &self.state {
            ParityState::Full => format!("{}: full parity", self.feature),
            ParityState::Partial { missing } => {
                format!("{}: partial (missing: {})", self.feature, missing.join(", "))
            }
            ParityState::Missing => format!("{}: not implemented", self.feature),
        }
    }
}

/// Return entries that are not at full parity.
pub fn parity_gaps(entries: &[ParityEntry]) -> Vec<&ParityEntry> {
    entries.iter().filter(|e| !e.is_full_parity()).collect()
}

/// True if all entries have full parity.
pub fn all_parity_green(entries: &[ParityEntry]) -> bool {
    entries.iter().all(|e| e.is_full_parity())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_parity_entry() {
        let e = ParityEntry::new("metrics-export", ParityState::Full);
        assert!(e.is_full_parity());
        assert!(e.describe().contains("full parity"));
    }

    #[test]
    fn partial_parity_entry() {
        let e = ParityEntry::new(
            "trace-propagation",
            ParityState::Partial { missing: vec!["go".into()] },
        );
        assert!(!e.is_full_parity());
        assert!(e.describe().contains("go"));
    }

    #[test]
    fn gaps_returned_correctly() {
        let entries = vec![
            ParityEntry::new("a", ParityState::Full),
            ParityEntry::new("b", ParityState::Missing),
        ];
        let gaps = parity_gaps(&entries);
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0].feature, "b");
    }
}
