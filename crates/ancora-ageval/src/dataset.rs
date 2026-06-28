//! Eval dataset format: typed sample container for behavior evaluations.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EvalSample {
    pub id: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl EvalSample {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[derive(Default)]
pub struct EvalDataset {
    pub name: String,
    pub samples: Vec<EvalSample>,
}

impl EvalDataset {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            samples: Vec::new(),
        }
    }

    pub fn add(&mut self, sample: EvalSample) {
        self.samples.push(sample);
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn by_tag<'a>(&'a self, tag: &str) -> Vec<&'a EvalSample> {
        self.samples.iter().filter(|s| s.has_tag(tag)).collect()
    }
}
