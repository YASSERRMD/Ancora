use crate::skill::SkillDescriptor;
use crate::registry::SkillRegistry;
use crate::error::SkillError;

/// Just-in-time skill loader that bounds context by loading on demand.
pub struct JitLoader {
    loaded: Vec<String>,
}

impl JitLoader {
    pub fn new() -> Self {
        Self { loaded: Vec::new() }
    }

    pub fn load_on_demand(&mut self, registry: &mut SkillRegistry, skill: SkillDescriptor) -> Result<(), SkillError> {
        if self.loaded.contains(&skill.name) {
            return Ok(());
        }
        self.loaded.push(skill.name.clone());
        registry.load(skill);
        Ok(())
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded.iter().any(|n| n == name)
    }

    pub fn loaded_count(&self) -> usize {
        self.loaded.len()
    }
}

impl Default for JitLoader {
    fn default() -> Self {
        Self::new()
    }
}
