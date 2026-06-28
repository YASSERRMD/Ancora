use crate::component::{Component, ComponentKind, License};

pub struct ComponentQuery {
    kind: Option<String>,
    license: Option<String>,
    supplier: Option<String>,
    open_source_only: bool,
}

impl ComponentQuery {
    pub fn new() -> Self {
        Self { kind: None, license: None, supplier: None, open_source_only: false }
    }

    pub fn kind(mut self, kind: ComponentKind) -> Self {
        self.kind = Some(format!("{}", kind));
        self
    }

    pub fn license(mut self, license: License) -> Self {
        self.license = Some(format!("{}", license));
        self
    }

    pub fn supplier(mut self, supplier: impl Into<String>) -> Self {
        self.supplier = Some(supplier.into());
        self
    }

    pub fn open_source_only(mut self) -> Self {
        self.open_source_only = true;
        self
    }

    pub fn run<'a>(&self, components: impl Iterator<Item = &'a Component>) -> Vec<&'a Component> {
        components.filter(|c| {
            if self.open_source_only && !c.is_open_source() { return false; }
            if let Some(k) = &self.kind {
                if format!("{}", c.kind) != *k { return false; }
            }
            if let Some(l) = &self.license {
                if format!("{}", c.license) != *l { return false; }
            }
            if let Some(s) = &self.supplier {
                if &c.supplier != s { return false; }
            }
            true
        }).collect()
    }
}
