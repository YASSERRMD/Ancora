/// Conformance kit for exporter extensions.

use std::collections::HashMap;

/// A record to be exported.
#[derive(Debug, Clone)]
pub struct Record {
    pub fields: HashMap<String, String>,
}

impl Record {
    pub fn new(fields: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) -> Self {
        Record {
            fields: fields
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

/// Trait that every exporter extension must satisfy.
pub trait Exporter {
    fn name(&self) -> &str;
    fn format(&self) -> &str;
    fn export(&self, records: &[Record]) -> Result<Vec<u8>, String>;
}

/// A single conformance check result.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

/// Kit that runs conformance checks against an [`Exporter`].
pub struct ExporterKit;

impl ExporterKit {
    pub fn new() -> Self {
        ExporterKit
    }

    pub fn run<E: Exporter>(&self, exporter: &E) -> Vec<CheckResult> {
        vec![
            self.check_name(exporter),
            self.check_format(exporter),
            self.check_export(exporter),
            self.check_empty_export(exporter),
        ]
    }

    fn check_name<E: Exporter>(&self, exporter: &E) -> CheckResult {
        if exporter.name().is_empty() {
            CheckResult {
                name: "exporter_name_nonempty".into(),
                passed: false,
                message: "Exporter name must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "exporter_name_nonempty".into(),
                passed: true,
                message: format!("Exporter name: {}", exporter.name()),
            }
        }
    }

    fn check_format<E: Exporter>(&self, exporter: &E) -> CheckResult {
        if exporter.format().is_empty() {
            CheckResult {
                name: "exporter_format_nonempty".into(),
                passed: false,
                message: "Exporter format must not be empty".into(),
            }
        } else {
            CheckResult {
                name: "exporter_format_nonempty".into(),
                passed: true,
                message: format!("Format: {}", exporter.format()),
            }
        }
    }

    fn check_export<E: Exporter>(&self, exporter: &E) -> CheckResult {
        let records = vec![
            Record::new([("key", "value"), ("count", "42")]),
            Record::new([("key", "other"), ("count", "7")]),
        ];
        match exporter.export(&records) {
            Ok(bytes) if !bytes.is_empty() => CheckResult {
                name: "exporter_export_nonempty".into(),
                passed: true,
                message: format!("Exported {} byte(s)", bytes.len()),
            },
            Ok(_) => CheckResult {
                name: "exporter_export_nonempty".into(),
                passed: false,
                message: "export() returned empty output".into(),
            },
            Err(e) => CheckResult {
                name: "exporter_export_nonempty".into(),
                passed: false,
                message: format!("export() errored: {e}"),
            },
        }
    }

    fn check_empty_export<E: Exporter>(&self, exporter: &E) -> CheckResult {
        match exporter.export(&[]) {
            Ok(_) => CheckResult {
                name: "exporter_handles_empty_input".into(),
                passed: true,
                message: "Handled empty record slice without error".into(),
            },
            Err(e) => CheckResult {
                name: "exporter_handles_empty_input".into(),
                passed: false,
                message: format!("export([]) errored: {e}"),
            },
        }
    }
}

impl Default for ExporterKit {
    fn default() -> Self {
        Self::new()
    }
}
