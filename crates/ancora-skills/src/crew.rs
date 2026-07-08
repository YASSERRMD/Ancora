use crate::error::SkillError;
use crate::registry::SkillRegistry;
use crate::skill::SkillDescriptor;

/// A composed crew of skills that run sequentially.
#[derive(Debug, Clone)]
pub struct Crew {
    pub name: String,
    pub skill_names: Vec<String>,
}

impl Crew {
    pub fn new(name: &str, skill_names: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            skill_names: skill_names.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn resolve<'a>(
        &self,
        registry: &'a SkillRegistry,
    ) -> Result<Vec<&'a SkillDescriptor>, SkillError> {
        self.skill_names
            .iter()
            .map(|n| registry.lookup(n))
            .collect()
    }
}
