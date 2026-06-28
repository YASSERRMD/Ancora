use crate::measurement::{Measurement, MeasurementKind};

pub struct MeasurementBuilder {
    id: String,
    kind: MeasurementKind,
    name: String,
    digest: String,
    tick: u64,
}

impl MeasurementBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: MeasurementKind::Application,
            name: name.into(),
            digest: String::new(),
            tick: 0,
        }
    }

    pub fn kind(mut self, kind: MeasurementKind) -> Self { self.kind = kind; self }
    pub fn digest(mut self, digest: impl Into<String>) -> Self { self.digest = digest.into(); self }
    pub fn tick(mut self, tick: u64) -> Self { self.tick = tick; self }

    pub fn build(self) -> Measurement {
        Measurement::new(self.id, self.kind, self.name, self.digest, self.tick)
    }
}
