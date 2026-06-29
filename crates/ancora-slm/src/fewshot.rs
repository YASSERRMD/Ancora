//! Few-shot injection tuned for small language models.
//!
//! Small models benefit significantly from seeing 2-5 high-quality examples
//! of the expected input/output format before the actual task.  This module
//! provides:
//!
//! - A [`FewShotExample`] type to store input/output pairs.
//! - A [`FewShotLibrary`] that stores examples indexed by task type.
//! - [`inject_few_shots`]: augments a prompt with the best-matching examples.
//! - Deterministic retrieval so replay always selects the same examples.

use serde::{Deserialize, Serialize};

/// A single few-shot example (input prompt → expected output).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotExample {
    /// Short tag identifying the task type this example belongs to.
    pub task_tag: String,
    /// The example input (what the user / caller would say).
    pub input: String,
    /// The ideal model output for the example input.
    pub output: String,
    /// Optional quality score (0.0 – 1.0); higher is preferred for selection.
    pub quality: f32,
}

impl FewShotExample {
    pub fn new(
        task_tag: impl Into<String>,
        input: impl Into<String>,
        output: impl Into<String>,
        quality: f32,
    ) -> Self {
        Self {
            task_tag: task_tag.into(),
            input: input.into(),
            output: output.into(),
            quality,
        }
    }
}

/// Storage for few-shot examples, indexed by task tag.
#[derive(Debug, Default, Clone)]
pub struct FewShotLibrary {
    examples: Vec<FewShotExample>,
}

impl FewShotLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an example to the library.
    pub fn add(&mut self, example: FewShotExample) {
        self.examples.push(example);
    }

    /// Retrieve up to `n` examples for `task_tag`, sorted by quality
    /// (descending).  Deterministic: ties broken by insertion order.
    pub fn retrieve(&self, task_tag: &str, n: usize) -> Vec<&FewShotExample> {
        let mut candidates: Vec<(usize, &FewShotExample)> = self
            .examples
            .iter()
            .enumerate()
            .filter(|(_, e)| e.task_tag == task_tag)
            .collect();

        // Sort by quality DESC, then by original index ASC for stable output.
        candidates.sort_by(|(ia, a), (ib, b)| {
            b.quality
                .partial_cmp(&a.quality)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(ia.cmp(ib))
        });

        candidates.into_iter().take(n).map(|(_, e)| e).collect()
    }

    /// Total number of examples stored.
    pub fn len(&self) -> usize {
        self.examples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.examples.is_empty()
    }
}

/// Format a few-shot block to prepend to a prompt.
///
/// Each example is formatted as:
/// ```text
/// Example 1:
/// Input: <input>
/// Output: <output>
/// ```
pub fn format_few_shot_block(examples: &[&FewShotExample]) -> String {
    if examples.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for (i, ex) in examples.iter().enumerate() {
        out.push_str(&format!(
            "Example {}:\nInput: {}\nOutput: {}\n\n",
            i + 1,
            ex.input.trim(),
            ex.output.trim()
        ));
    }
    out
}

/// Augment a task prompt by prepending the best few-shot examples from the
/// library.  The examples are selected deterministically by quality score.
///
/// If no examples are found for `task_tag`, returns the original prompt unchanged.
pub fn inject_few_shots(
    prompt: &str,
    library: &FewShotLibrary,
    task_tag: &str,
    max_examples: usize,
) -> String {
    let examples = library.retrieve(task_tag, max_examples);
    if examples.is_empty() {
        return prompt.to_string();
    }
    let block = format_few_shot_block(&examples);
    format!(
        "Here are some examples of the expected format:\n\n{}Now complete the following:\n{}",
        block, prompt
    )
}
