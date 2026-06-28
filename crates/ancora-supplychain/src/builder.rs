use crate::component::{Component, ComponentKind, License};

pub struct ComponentBuilder {
    id: String,
    name: String,
    version: String,
    kind: ComponentKind,
    license: License,
    supplier: String,
    digest: String,
}

impl ComponentBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            kind: ComponentKind::Library,
            license: License::Unknown,
            supplier: String::new(),
            digest: String::new(),
        }
    }

    pub fn kind(mut self, kind: ComponentKind) -> Self { self.kind = kind; self }
    pub fn license(mut self, license: License) -> Self { self.license = license; self }
    pub fn supplier(mut self, supplier: impl Into<String>) -> Self { self.supplier = supplier.into(); self }
    pub fn digest(mut self, digest: impl Into<String>) -> Self { self.digest = digest.into(); self }

    pub fn build(self) -> Component {
        Component::new(self.id, self.name, self.version, self.kind, self.license, self.supplier, self.digest)
    }
}
