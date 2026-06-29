//! Documentation metadata for the on-device runtime.
//!
//! Provides structured access to the embedded doc topics so tools
//! can enumerate, validate, and surface the documentation at runtime.

use serde::{Deserialize, Serialize};

/// A documentation topic shipped with the crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTopic {
    /// Machine-readable slug (matches the filename without extension).
    pub slug: &'static str,
    /// Human-readable title.
    pub title: &'static str,
    /// Short description of the topic.
    pub description: &'static str,
    /// Relative path inside the crate's `docs/` directory.
    pub path: &'static str,
}

/// All documentation topics shipped with `ancora-ondev`.
pub const DOC_TOPICS: &[DocTopic] = &[
    DocTopic {
        slug: "on-device-runtime-guide",
        title: "On-Device Runtime Guide",
        description: "Overview of the on-device runtime, supported targets, and quick-start.",
        path: "docs/on-device-runtime-guide.md",
    },
    DocTopic {
        slug: "mobile-integration",
        title: "Mobile Integration",
        description: "How to embed ancora-ondev into Android (JNI) and iOS (C-ABI) apps.",
        path: "docs/mobile-integration.md",
    },
    DocTopic {
        slug: "footprint-tuning",
        title: "Footprint Tuning",
        description: "Techniques for reducing binary and memory footprint on constrained devices.",
        path: "docs/footprint-tuning.md",
    },
];

/// Look up a documentation topic by slug.
pub fn find_topic(slug: &str) -> Option<&'static DocTopic> {
    DOC_TOPICS.iter().find(|t| t.slug == slug)
}

/// Return all topic slugs.
pub fn all_slugs() -> Vec<&'static str> {
    DOC_TOPICS.iter().map(|t| t.slug).collect()
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn all_topics_have_unique_slugs() {
        let slugs = all_slugs();
        let mut seen = std::collections::HashSet::new();
        for slug in &slugs {
            assert!(seen.insert(*slug), "duplicate slug: {}", slug);
        }
    }

    #[test]
    fn find_topic_returns_correct_entry() {
        let t = find_topic("mobile-integration").unwrap();
        assert_eq!(t.title, "Mobile Integration");
    }

    #[test]
    fn find_topic_unknown_returns_none() {
        assert!(find_topic("does-not-exist").is_none());
    }
}
