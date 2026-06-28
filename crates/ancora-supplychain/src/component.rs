use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentKind {
    Library,
    Binary,
    Container,
    OsPackage,
    Framework,
    Service,
}

impl fmt::Display for ComponentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ComponentKind::Library => "LIBRARY",
            ComponentKind::Binary => "BINARY",
            ComponentKind::Container => "CONTAINER",
            ComponentKind::OsPackage => "OS_PACKAGE",
            ComponentKind::Framework => "FRAMEWORK",
            ComponentKind::Service => "SERVICE",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum License {
    Mit,
    Apache2,
    Gpl3,
    Bsd2,
    Bsd3,
    Proprietary,
    Unknown,
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            License::Mit => "MIT",
            License::Apache2 => "Apache-2.0",
            License::Gpl3 => "GPL-3.0",
            License::Bsd2 => "BSD-2-Clause",
            License::Bsd3 => "BSD-3-Clause",
            License::Proprietary => "PROPRIETARY",
            License::Unknown => "UNKNOWN",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Clone)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub version: String,
    pub kind: ComponentKind,
    pub license: License,
    pub supplier: String,
    pub digest: String,
    pub metadata: HashMap<String, String>,
}

impl Component {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
        kind: ComponentKind,
        license: License,
        supplier: impl Into<String>,
        digest: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            kind,
            license,
            supplier: supplier.into(),
            digest: digest.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn is_open_source(&self) -> bool {
        !matches!(self.license, License::Proprietary | License::Unknown)
    }
}
