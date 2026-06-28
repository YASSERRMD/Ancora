use crate::control::{ComplianceControl, ControlStatus};
use crate::framework::Framework;

#[derive(Default)]
pub struct ControlQuery {
    pub framework: Option<Framework>,
    pub status: Option<ControlStatus>,
    pub has_evidence: Option<bool>,
}

impl ControlQuery {
    pub fn new() -> Self { Self::default() }
    pub fn framework(mut self, f: Framework) -> Self { self.framework = Some(f); self }
    pub fn status(mut self, s: ControlStatus) -> Self { self.status = Some(s); self }
    pub fn has_evidence(mut self, v: bool) -> Self { self.has_evidence = Some(v); self }

    pub fn run<'a>(&self, controls: impl Iterator<Item = &'a ComplianceControl>) -> Vec<&'a ComplianceControl> {
        controls.filter(|c| {
            if let Some(ref f) = self.framework { if &c.framework != f { return false; } }
            if let Some(ref s) = self.status { if &c.status != s { return false; } }
            if let Some(has_ev) = self.has_evidence {
                let actual = !c.evidence_ids.is_empty();
                if actual != has_ev { return false; }
            }
            true
        }).collect()
    }
}
