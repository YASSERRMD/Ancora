/// Semantic version in major.minor.patch form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Author information attached to a catalog entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

impl Author {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: None,
            url: None,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// SPDX-style license identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct License(pub String);

impl License {
    pub fn new(spdx: impl Into<String>) -> Self {
        Self(spdx.into())
    }

    pub fn apache2() -> Self {
        Self("Apache-2.0".into())
    }

    pub fn mit() -> Self {
        Self("MIT".into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Metadata common to every catalog entry type.
#[derive(Debug, Clone)]
pub struct Metadata {
    pub version: Version,
    pub author: Author,
    pub license: License,
    pub tags: Vec<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
}

impl Metadata {
    pub fn new(version: Version, author: Author, license: License) -> Self {
        Self {
            version,
            author,
            license,
            tags: Vec::new(),
            homepage: None,
            repository: None,
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_homepage(mut self, url: impl Into<String>) -> Self {
        self.homepage = Some(url.into());
        self
    }

    pub fn with_repository(mut self, url: impl Into<String>) -> Self {
        self.repository = Some(url.into());
        self
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
