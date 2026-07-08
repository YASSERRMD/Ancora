//! Index of extension examples shipped with Ancora.
//!
//! Provides a searchable index of example crates so documentation
//! tooling can emit cross-references automatically.

/// Category of an example.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExampleCategory {
    Plugin,
    CliCommand,
    GraphNode,
    FrameworkAdapter,
    Recipe,
}

impl std::fmt::Display for ExampleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Plugin => "plugin",
            Self::CliCommand => "cli-command",
            Self::GraphNode => "graph-node",
            Self::FrameworkAdapter => "framework-adapter",
            Self::Recipe => "recipe",
        };
        write!(f, "{label}")
    }
}

/// Metadata for a single example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Example {
    pub name: &'static str,
    pub category: ExampleCategory,
    pub description: &'static str,
    pub path: &'static str,
}

/// Returns the complete example index.
pub fn example_index() -> Vec<Example> {
    vec![
        Example {
            name: "hello-plugin",
            category: ExampleCategory::Plugin,
            description: "Minimal plugin that logs lifecycle events",
            path: "examples/hello-plugin",
        },
        Example {
            name: "counter-node",
            category: ExampleCategory::GraphNode,
            description: "Graph node that counts processed items",
            path: "examples/counter-node",
        },
        Example {
            name: "greet-cli",
            category: ExampleCategory::CliCommand,
            description: "CLI subcommand that greets the user",
            path: "examples/greet-cli",
        },
        Example {
            name: "stub-adapter",
            category: ExampleCategory::FrameworkAdapter,
            description: "No-op framework adapter for testing",
            path: "examples/stub-adapter",
        },
        Example {
            name: "copy-recipe",
            category: ExampleCategory::Recipe,
            description: "Workflow recipe that copies a file between two paths",
            path: "examples/copy-recipe",
        },
    ]
}

/// Filter examples by category.
pub fn by_category(cat: &ExampleCategory) -> Vec<Example> {
    example_index()
        .into_iter()
        .filter(|e| &e.category == cat)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_is_non_empty() {
        assert!(!example_index().is_empty());
    }

    #[test]
    fn all_examples_have_paths() {
        for ex in example_index() {
            assert!(!ex.path.is_empty(), "{} has no path", ex.name);
        }
    }

    #[test]
    fn filter_by_plugin_category() {
        let plugins = by_category(&ExampleCategory::Plugin);
        assert!(!plugins.is_empty());
        assert!(plugins
            .iter()
            .all(|e| e.category == ExampleCategory::Plugin));
    }
}
