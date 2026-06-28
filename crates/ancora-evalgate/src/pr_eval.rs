/// Per-PR eval selection.
///
/// Determines which eval datasets and metrics should run for a given PR
/// based on labels, changed files, or an explicit allow-list.

/// A selector rule maps a file-path prefix to a list of dataset names.
#[derive(Debug, Clone)]
pub struct SelectorRule {
    /// File path prefix that triggers this rule (e.g. "crates/ancora-cost").
    pub path_prefix: String,
    /// Eval datasets that should run when the prefix matches.
    pub datasets: Vec<String>,
}

impl SelectorRule {
    pub fn new(path_prefix: impl Into<String>, datasets: Vec<String>) -> Self {
        Self {
            path_prefix: path_prefix.into(),
            datasets,
        }
    }

    pub fn matches(&self, changed_file: &str) -> bool {
        changed_file.starts_with(&self.path_prefix)
    }
}

/// Selects the union of eval datasets that should run for a PR.
///
/// `changed_files` - list of file paths modified by the PR.
/// `rules`         - selector rules to evaluate.
/// `always_run`    - datasets that always run regardless of changed files.
pub fn select_datasets(
    changed_files: &[&str],
    rules: &[SelectorRule],
    always_run: &[String],
) -> Vec<String> {
    let mut datasets: Vec<String> = always_run.to_vec();

    for rule in rules {
        for &file in changed_files {
            if rule.matches(file) {
                for ds in &rule.datasets {
                    if !datasets.contains(ds) {
                        datasets.push(ds.clone());
                    }
                }
                break;
            }
        }
    }

    datasets.sort();
    datasets.dedup();
    datasets
}
