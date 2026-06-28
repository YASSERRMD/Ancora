/// A single migration step.
pub struct Migration {
    pub version: u32,
    pub description: String,
    pub up: Box<dyn Fn() -> Result<(), String>>,
    pub down: Box<dyn Fn() -> Result<(), String>>,
}

impl Migration {
    pub fn new(
        version: u32,
        description: &str,
        up: impl Fn() -> Result<(), String> + 'static,
        down: impl Fn() -> Result<(), String> + 'static,
    ) -> Self {
        Self {
            version,
            description: description.to_string(),
            up: Box::new(up),
            down: Box::new(down),
        }
    }

    pub fn apply(&self) -> Result<(), String> {
        (self.up)()
    }

    pub fn rollback(&self) -> Result<(), String> {
        (self.down)()
    }
}
