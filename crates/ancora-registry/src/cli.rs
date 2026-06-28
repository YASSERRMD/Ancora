use crate::fetch::FetchResult;
use crate::publish::PublishEntry;
use crate::search::SearchQuery;
use crate::service::RegistryService;
use crate::versioning::Version;

/// A command understood by the registry CLI.
#[derive(Debug, Clone)]
pub enum CliCommand {
    /// Publish a new entry.
    Publish {
        name: String,
        version: Version,
        payload: Vec<u8>,
        publisher: String,
        signature: Option<String>,
    },
    /// Fetch a specific version of an entry.
    Fetch { name: String, version: Version },
    /// Search for entries by term.
    Search { term: String },
    /// List all versions of a named entry.
    Versions { name: String },
}

/// The output produced by executing a CLI command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliOutput {
    /// Command succeeded with an optional human-readable message.
    Ok(String),
    /// Command failed with an error message.
    Err(String),
    /// Fetch succeeded; contains the payload.
    Payload(Vec<u8>),
    /// Search/version listing results.
    Lines(Vec<String>),
}

impl CliOutput {
    pub fn is_ok(&self) -> bool {
        !matches!(self, Self::Err(_))
    }
}

/// Execute a single CLI command against a registry service.
pub fn run_command(svc: &mut RegistryService, cmd: CliCommand) -> CliOutput {
    match cmd {
        CliCommand::Publish {
            name,
            version,
            payload,
            publisher,
            signature,
        } => {
            let mut entry = PublishEntry::new(name, version, payload, publisher);
            if let Some(sig) = signature {
                entry = entry.with_signature(sig);
            }
            match svc.publish(entry) {
                Ok(()) => CliOutput::Ok("published".to_string()),
                Err(e) => CliOutput::Err(e.to_string()),
            }
        }
        CliCommand::Fetch { name, version } => match svc.fetch(&name, &version) {
            FetchResult::Found(data) => CliOutput::Payload(data),
            FetchResult::NotFound => CliOutput::Err(format!(
                "entry '{name}' version '{version}' not found"
            )),
        },
        CliCommand::Search { term } => {
            let query = SearchQuery::new(term);
            let hits = svc.search(&query);
            let lines = hits
                .into_iter()
                .map(|h| {
                    let latest = h
                        .latest
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string());
                    format!("{} (latest: {}, versions: {})", h.name, latest, h.version_count)
                })
                .collect();
            CliOutput::Lines(lines)
        }
        CliCommand::Versions { name } => {
            let versions = svc.list_versions(&name);
            if versions.is_empty() {
                CliOutput::Err(format!("no versions found for '{name}'"))
            } else {
                let lines = versions.iter().map(|v| v.to_string()).collect();
                CliOutput::Lines(lines)
            }
        }
    }
}

/// Parse a simple command string into a `CliCommand`.
///
/// Syntax:
///   publish <name> <version> <publisher>
///   fetch <name> <version>
///   search <term>
///   versions <name>
pub fn parse_command(input: &str) -> Result<CliCommand, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        ["publish", name, version_str, publisher] => {
            let version = Version::parse(version_str)
                .map_err(|e| e.to_string())?;
            Ok(CliCommand::Publish {
                name: name.to_string(),
                version,
                payload: name.as_bytes().to_vec(),
                publisher: publisher.to_string(),
                signature: None,
            })
        }
        ["fetch", name, version_str] => {
            let version = Version::parse(version_str)
                .map_err(|e| e.to_string())?;
            Ok(CliCommand::Fetch {
                name: name.to_string(),
                version,
            })
        }
        ["search", term] => Ok(CliCommand::Search {
            term: term.to_string(),
        }),
        ["versions", name] => Ok(CliCommand::Versions {
            name: name.to_string(),
        }),
        _ => Err(format!("unknown command: '{input}'")),
    }
}
