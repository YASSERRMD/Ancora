/// A single evaluation example: inputs, expected output, and metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct Example {
    /// Unique identifier for this example.
    pub id: String,
    /// The input provided to the agent/model.
    pub input: String,
    /// The expected output or reference answer.
    pub expected: String,
    /// Arbitrary metadata key-value pairs.
    pub metadata: std::collections::HashMap<String, String>,
}

impl Example {
    pub fn new(id: impl Into<String>, input: impl Into<String>, expected: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            input: input.into(),
            expected: expected.into(),
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// A versioned collection of evaluation examples.
#[derive(Debug, Clone)]
pub struct Dataset {
    /// Human-readable name of the dataset.
    pub name: String,
    /// Semantic version string (e.g., "1.0.0").
    pub version: String,
    /// The examples in this dataset.
    pub examples: Vec<Example>,
}

impl Dataset {
    /// Create a new empty dataset.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            examples: Vec::new(),
        }
    }

    /// Add an example to the dataset.
    pub fn add(&mut self, example: Example) {
        self.examples.push(example);
    }

    /// Return the number of examples.
    pub fn len(&self) -> usize {
        self.examples.len()
    }

    /// Return true if the dataset has no examples.
    pub fn is_empty(&self) -> bool {
        self.examples.is_empty()
    }

    /// Load a dataset from a simple CSV-like text format:
    /// Each line: `id,input,expected`
    pub fn from_csv(name: impl Into<String>, version: impl Into<String>, csv: &str) -> Result<Self, String> {
        let mut dataset = Dataset::new(name, version);
        for (line_no, line) in csv.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.splitn(3, ',').collect();
            if parts.len() < 3 {
                return Err(format!("Line {}: expected 3 fields, got {}", line_no + 1, parts.len()));
            }
            dataset.add(Example::new(parts[0].trim(), parts[1].trim(), parts[2].trim()));
        }
        Ok(dataset)
    }
}
