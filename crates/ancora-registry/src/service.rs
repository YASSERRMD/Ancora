use std::collections::HashMap;

use crate::access_control::{AccessPolicy, AccessResult};
use crate::airgap::AirgapMode;
use crate::fetch::FetchResult;
use crate::publish::{PublishEntry, PublishError};
use crate::search::{SearchHit, SearchQuery};
use crate::signature::SignatureStore;
use crate::versioning::{Version, VersionList};

/// Configuration for the registry service.
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Name of this registry instance.
    pub name: String,
    /// Whether unsigned entries are accepted.
    pub strict_signatures: bool,
    /// Operating mode: online or air-gapped.
    pub airgap_mode: AirgapMode,
    /// Access policy applied on publish operations.
    pub access_policy: AccessPolicy,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            strict_signatures: false,
            airgap_mode: AirgapMode::Online,
            access_policy: AccessPolicy::Open,
        }
    }
}

/// In-memory store backing the registry service.
#[derive(Debug, Default)]
struct Store {
    // Maps (name, version) -> entry bytes
    entries: HashMap<(String, Version), Vec<u8>>,
    // Maps name -> list of versions, sorted ascending
    versions: HashMap<String, VersionList>,
}

impl Store {
    fn insert(&mut self, name: String, version: Version, payload: Vec<u8>) {
        self.versions
            .entry(name.clone())
            .or_default()
            .add(version.clone());
        self.entries.insert((name, version), payload);
    }

    fn get(&self, name: &str, version: &Version) -> Option<&Vec<u8>> {
        self.entries.get(&(name.to_string(), version.clone()))
    }

    fn versions_of(&self, name: &str) -> Option<&VersionList> {
        self.versions.get(name)
    }

    fn all_names(&self) -> impl Iterator<Item = &String> {
        self.versions.keys()
    }
}

/// The registry service: a self-contained, in-process registry.
pub struct RegistryService {
    pub config: RegistryConfig,
    store: Store,
    pub signatures: SignatureStore,
}

impl RegistryService {
    /// Create a new registry with the given configuration.
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            config,
            store: Store::default(),
            signatures: SignatureStore::default(),
        }
    }

    /// Publish an entry to the registry.
    pub fn publish(&mut self, entry: PublishEntry) -> Result<(), PublishError> {
        // Access control check.
        if let AccessResult::Denied(reason) = self.config.access_policy.check(&entry.publisher) {
            return Err(PublishError::AccessDenied(reason));
        }

        // Signature check when strict mode is enabled.
        if self.config.strict_signatures {
            match &entry.signature {
                None => return Err(PublishError::MissingSignature),
                Some(sig) => {
                    if !self.signatures.verify(&entry.name, &entry.version, sig) {
                        return Err(PublishError::InvalidSignature);
                    }
                }
            }
        } else if let Some(sig) = &entry.signature {
            // Store even if not required.
            self.signatures
                .store(entry.name.clone(), entry.version.clone(), sig.clone());
        }

        self.store
            .insert(entry.name.clone(), entry.version.clone(), entry.payload);
        Ok(())
    }

    /// Fetch an entry from the registry.
    pub fn fetch(&self, name: &str, version: &Version) -> FetchResult {
        match self.store.get(name, version) {
            Some(data) => FetchResult::Found(data.clone()),
            None => FetchResult::NotFound,
        }
    }

    /// Search for entries matching the query.
    pub fn search(&self, query: &SearchQuery) -> Vec<SearchHit> {
        let q = query.term.to_lowercase();
        self.store
            .all_names()
            .filter(|name| name.to_lowercase().contains(&q))
            .map(|name| {
                let versions = self
                    .store
                    .versions_of(name)
                    .map(|vl| vl.list().to_vec())
                    .unwrap_or_default();
                SearchHit {
                    name: name.clone(),
                    latest: versions.last().cloned(),
                    version_count: versions.len(),
                }
            })
            .collect()
    }

    /// List all versions of a named entry.
    pub fn list_versions(&self, name: &str) -> Vec<Version> {
        self.store
            .versions_of(name)
            .map(|vl| vl.list().to_vec())
            .unwrap_or_default()
    }
}
