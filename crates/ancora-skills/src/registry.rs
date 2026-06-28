use std::collections::HashMap;
use crate::skill::SkillDescriptor;
use crate::error::SkillError;

/// Registry of loaded skills with versioned lookup.
#[derive(Debug, Default)]
pub struct SkillRegistry {
    skills: HashMap<String, Vec<SkillDescriptor>>,
}

impl SkillRegistry {
    pub fn load(&mut self, skill: SkillDescriptor) {
        self.skills.entry(skill.name.clone()).or_default().push(skill);
    }

    pub fn find(&self, name: &str) -> Option<&SkillDescriptor> {
        self.skills.get(name)?.iter().max_by_key(|s| s.version)
    }

    pub fn find_version(&self, name: &str, version: u32) -> Option<&SkillDescriptor> {
        self.skills.get(name)?.iter().find(|s| s.version == version)
    }

    pub fn by_tag(&self, tag: &str) -> Vec<&SkillDescriptor> {
        self.skills.values()
            .flat_map(|v| v.iter())
            .filter(|s| s.has_tag(tag))
            .collect()
    }

    pub fn lookup(&self, name: &str) -> Result<&SkillDescriptor, SkillError> {
        self.find(name).ok_or_else(|| SkillError::NotFound(name.to_string()))
    }

    pub fn len(&self) -> usize {
        self.skills.len()
    }
}
