use crate::measurement::{Measurement, MeasurementKind};

pub struct MeasurementQuery {
    kind: Option<String>,
    digest_prefix: Option<String>,
    name_contains: Option<String>,
}

impl MeasurementQuery {
    pub fn new() -> Self {
        Self { kind: None, digest_prefix: None, name_contains: None }
    }

    pub fn kind(mut self, kind: MeasurementKind) -> Self {
        self.kind = Some(format!("{}", kind));
        self
    }

    pub fn digest_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.digest_prefix = Some(prefix.into());
        self
    }

    pub fn name_contains(mut self, fragment: impl Into<String>) -> Self {
        self.name_contains = Some(fragment.into());
        self
    }

    pub fn run<'a>(&self, measurements: impl Iterator<Item = &'a Measurement>) -> Vec<&'a Measurement> {
        measurements.filter(|m| {
            if let Some(k) = &self.kind {
                if format!("{}", m.kind) != *k { return false; }
            }
            if let Some(prefix) = &self.digest_prefix {
                if !m.digest.starts_with(prefix.as_str()) { return false; }
            }
            if let Some(frag) = &self.name_contains {
                if !m.name.contains(frag.as_str()) { return false; }
            }
            true
        }).collect()
    }
}
