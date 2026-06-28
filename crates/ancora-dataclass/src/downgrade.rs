use crate::label::SensitivityLevel;
use crate::record::DataRecord;

#[derive(Debug, PartialEq, Eq)]
pub enum DowngradeResult {
    Downgraded,
    AlreadyAtOrBelow,
    Denied(String),
}

pub struct DowngradePolicy {
    pub min_level: SensitivityLevel,
}

impl DowngradePolicy {
    pub fn new(min_level: SensitivityLevel) -> Self { Self { min_level } }

    pub fn apply(&self, record: &mut DataRecord, target: SensitivityLevel) -> DowngradeResult {
        if target.is_above(&record.level) {
            return DowngradeResult::Denied(format!(
                "target {} is above current level {}; use upgrade instead",
                target, record.level
            ));
        }
        if target.numeric() < self.min_level.numeric() {
            return DowngradeResult::Denied(format!(
                "target {} is below policy minimum {}",
                target, self.min_level
            ));
        }
        if record.level == target {
            return DowngradeResult::AlreadyAtOrBelow;
        }
        record.level = target;
        DowngradeResult::Downgraded
    }
}
