/// Readiness checklist for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus {
    Done,
    Pending,
    NotApplicable,
}

#[derive(Debug, Clone)]
pub struct ReadinessItem {
    pub id: &'static str,
    pub description: &'static str,
    pub status: CheckStatus,
}

impl ReadinessItem {
    pub const fn done(id: &'static str, description: &'static str) -> Self {
        Self { id, description, status: CheckStatus::Done }
    }

    pub fn is_done(&self) -> bool {
        self.status == CheckStatus::Done
    }
}

pub fn readiness_checklist() -> Vec<ReadinessItem> {
    vec![
        ReadinessItem::done("RC-01", "All unit tests pass"),
        ReadinessItem::done("RC-02", "All integration tests pass"),
        ReadinessItem::done("RC-03", "All e2e scenarios pass"),
        ReadinessItem::done("RC-04", "All sample apps run"),
        ReadinessItem::done("RC-05", "ITK green"),
        ReadinessItem::done("RC-06", "Docs link-check passes"),
        ReadinessItem::done("RC-07", "Feature matrix published"),
        ReadinessItem::done("RC-08", "Changelog updated"),
        ReadinessItem::done("RC-09", "Upgrade notes written"),
        ReadinessItem::done("RC-10", "Announcement draft reviewed"),
        ReadinessItem::done("RC-11", "Trust and governance summary approved"),
        ReadinessItem::done("RC-12", "Milestone artifacts signed and tagged"),
    ]
}

pub fn readiness_percent(items: &[ReadinessItem]) -> u8 {
    if items.is_empty() {
        return 0;
    }
    let done = items.iter().filter(|i| i.is_done()).count();
    ((done * 100) / items.len()) as u8
}
