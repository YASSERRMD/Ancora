/// Plugin end-to-end utilities: template authoring and lifecycle management.

#[derive(Debug, Clone, PartialEq)]
pub struct PluginTemplate {
    pub name: String,
    pub version: String,
    pub description: String,
    pub entry_point: String,
}

impl PluginTemplate {
    pub fn new(name: &str, version: &str, description: &str, entry_point: &str) -> Self {
        PluginTemplate {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            entry_point: entry_point.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.version.is_empty() && !self.entry_point.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PluginState {
    Created,
    Compiled,
    Installed,
    Running,
    Stopped,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub template: PluginTemplate,
    pub state: PluginState,
    pub id: u64,
}

impl Plugin {
    pub fn from_template(template: PluginTemplate, id: u64) -> Option<Plugin> {
        if !template.is_valid() {
            return None;
        }
        Some(Plugin {
            template,
            state: PluginState::Created,
            id,
        })
    }

    pub fn compile(&mut self) -> Result<(), String> {
        match self.state {
            PluginState::Created => {
                self.state = PluginState::Compiled;
                Ok(())
            }
            _ => Err(format!("cannot compile plugin in state {:?}", self.state)),
        }
    }

    pub fn install(&mut self) -> Result<(), String> {
        match self.state {
            PluginState::Compiled => {
                self.state = PluginState::Installed;
                Ok(())
            }
            _ => Err(format!("cannot install plugin in state {:?}", self.state)),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        match self.state {
            PluginState::Installed => {
                self.state = PluginState::Running;
                Ok(())
            }
            _ => Err(format!("cannot start plugin in state {:?}", self.state)),
        }
    }

    pub fn stop(&mut self) -> Result<(), String> {
        match self.state {
            PluginState::Running => {
                self.state = PluginState::Stopped;
                Ok(())
            }
            _ => Err(format!("cannot stop plugin in state {:?}", self.state)),
        }
    }
}
