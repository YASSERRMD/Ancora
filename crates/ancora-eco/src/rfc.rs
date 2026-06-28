/// Status of an RFC in the extension ecosystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RfcStatus {
    Draft,
    FinalCommentPeriod,
    Accepted,
    Rejected,
    Withdrawn,
    Implemented,
}

/// An RFC proposing a change to the extension API or governance.
#[derive(Debug, Clone)]
pub struct Rfc {
    pub number: u32,
    pub title: String,
    pub summary: String,
    pub status: RfcStatus,
}

impl Rfc {
    pub fn new(number: u32, title: impl Into<String>, summary: impl Into<String>) -> Self {
        Rfc {
            number,
            title: title.into(),
            summary: summary.into(),
            status: RfcStatus::Draft,
        }
    }

    /// Returns true if this RFC can transition to `next`.
    pub fn can_transition_to(&self, next: &RfcStatus) -> bool {
        use RfcStatus::*;
        matches!(
            (&self.status, next),
            (Draft, FinalCommentPeriod)
                | (FinalCommentPeriod, Accepted)
                | (FinalCommentPeriod, Rejected)
                | (Accepted, Implemented)
                | (Draft, Withdrawn)
                | (FinalCommentPeriod, Withdrawn)
        )
    }

    /// Transition the RFC status.
    pub fn transition(&mut self, next: RfcStatus) -> Result<(), String> {
        if self.can_transition_to(&next) {
            self.status = next;
            Ok(())
        } else {
            Err(format!(
                "invalid RFC transition from {:?} to {:?}",
                self.status, next
            ))
        }
    }
}

/// Registry of all RFCs in the ecosystem.
#[derive(Debug, Default)]
pub struct RfcRegistry {
    rfcs: Vec<Rfc>,
}

impl RfcRegistry {
    pub fn new() -> Self {
        RfcRegistry { rfcs: Vec::new() }
    }

    pub fn submit(&mut self, rfc: Rfc) {
        self.rfcs.push(rfc);
    }

    pub fn get_mut(&mut self, number: u32) -> Option<&mut Rfc> {
        self.rfcs.iter_mut().find(|r| r.number == number)
    }

    pub fn accepted(&self) -> Vec<&Rfc> {
        self.rfcs
            .iter()
            .filter(|r| r.status == RfcStatus::Accepted || r.status == RfcStatus::Implemented)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_transitions_correctly() {
        let mut rfc = Rfc::new(1, "Add hook API", "Introduces a hook-based extension API");
        rfc.transition(RfcStatus::FinalCommentPeriod).unwrap();
        rfc.transition(RfcStatus::Accepted).unwrap();
        assert_eq!(rfc.status, RfcStatus::Accepted);
    }

    #[test]
    fn invalid_rfc_transition_rejected() {
        let mut rfc = Rfc::new(2, "Remove legacy API", "Removes old extension entrypoints");
        let result = rfc.transition(RfcStatus::Accepted);
        assert!(result.is_err());
    }
}
