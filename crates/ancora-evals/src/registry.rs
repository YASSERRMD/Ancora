use crate::grader::{Grader, Score};
use std::collections::HashMap;

/// A boxed, named grader that can be stored in the registry.
pub struct BoxedGrader(pub Box<dyn Grader + Send + Sync>);

impl BoxedGrader {
    pub fn grade(&self, candidate: &str, expected: &str) -> Score {
        self.0.grade(candidate, expected)
    }

    pub fn name(&self) -> &str {
        self.0.name()
    }
}

/// Central registry for custom graders.
///
/// Users register graders by name and then look them up at eval time.
#[derive(Default)]
pub struct GraderRegistry {
    graders: HashMap<String, BoxedGrader>,
}

impl GraderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a grader under a given name.
    pub fn register<G>(&mut self, name: impl Into<String>, grader: G)
    where
        G: Grader + Send + Sync + 'static,
    {
        self.graders
            .insert(name.into(), BoxedGrader(Box::new(grader)));
    }

    /// Look up a grader by name.
    pub fn get(&self, name: &str) -> Option<&BoxedGrader> {
        self.graders.get(name)
    }

    /// List all registered grader names.
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.graders.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    /// Grade using the named grader. Returns an error string if not found.
    pub fn grade(
        &self,
        grader_name: &str,
        candidate: &str,
        expected: &str,
    ) -> Result<Score, String> {
        match self.graders.get(grader_name) {
            Some(g) => Ok(g.grade(candidate, expected)),
            None => Err(format!("Grader '{}' not found in registry", grader_name)),
        }
    }
}
