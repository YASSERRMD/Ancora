/// A journaled record of a skill invocation for replay.
#[derive(Debug, Clone)]
pub struct SkillInvocationRecord {
    pub tick: u64,
    pub skill_name: String,
    pub version: u32,
    pub node_id: String,
}

/// Append-only journal of skill invocations.
#[derive(Debug, Default)]
pub struct SkillJournal {
    records: Vec<SkillInvocationRecord>,
}

impl SkillJournal {
    pub fn record(&mut self, tick: u64, skill_name: &str, version: u32, node_id: &str) {
        self.records.push(SkillInvocationRecord {
            tick,
            skill_name: skill_name.to_string(),
            version,
            node_id: node_id.to_string(),
        });
    }

    pub fn records(&self) -> &[SkillInvocationRecord] {
        &self.records
    }

    pub fn records_for_skill(&self, name: &str) -> Vec<&SkillInvocationRecord> {
        self.records
            .iter()
            .filter(|r| r.skill_name == name)
            .collect()
    }

    pub fn replay(&self) -> Vec<(&str, u32)> {
        self.records
            .iter()
            .map(|r| (r.skill_name.as_str(), r.version))
            .collect()
    }
}
