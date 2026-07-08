/// Known limitations for the ecosystem milestone.
#[derive(Debug, Clone)]
pub struct Limitation {
    pub id: &'static str,
    pub summary: &'static str,
    pub workaround: Option<&'static str>,
    pub target_release: Option<&'static str>,
}

impl Limitation {
    pub const fn new(
        id: &'static str,
        summary: &'static str,
        workaround: Option<&'static str>,
        target_release: Option<&'static str>,
    ) -> Self {
        Self {
            id,
            summary,
            workaround,
            target_release,
        }
    }

    pub fn has_workaround(&self) -> bool {
        self.workaround.is_some()
    }
}

pub fn known_limitations() -> Vec<Limitation> {
    vec![
        Limitation::new(
            "LIM-001",
            "Plugin hot-reload requires a brief pause between reloads",
            Some("Wait 500ms between consecutive reloads"),
            Some("v0.7.0"),
        ),
        Limitation::new(
            "LIM-002",
            "gRPC streaming backpressure not yet propagated to callers",
            Some("Use chunked polling as interim approach"),
            Some("v0.7.0"),
        ),
        Limitation::new(
            "LIM-003",
            "Python FFI does not support async generators",
            Some("Use synchronous iteration with manual polling"),
            Some("v0.8.0"),
        ),
        Limitation::new(
            "LIM-004",
            "Catalog search results limited to 100 entries per page",
            None,
            Some("v0.7.0"),
        ),
    ]
}
