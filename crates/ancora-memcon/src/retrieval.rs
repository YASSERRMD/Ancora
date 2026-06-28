/// Checks that key facts survive consolidation by keyword search.
pub struct RetrievalChecker;

impl RetrievalChecker {
    /// Returns true if all required keywords appear in retained content.
    pub fn check(retained_contents: &[String], required_keywords: &[&str]) -> bool {
        for kw in required_keywords {
            let found = retained_contents.iter().any(|c| c.contains(kw));
            if !found {
                return false;
            }
        }
        true
    }

    /// Returns the keywords that are missing from retained content.
    pub fn missing_keywords<'a>(
        retained_contents: &[String],
        required_keywords: &[&'a str],
    ) -> Vec<&'a str> {
        required_keywords
            .iter()
            .copied()
            .filter(|kw| !retained_contents.iter().any(|c| c.contains(kw)))
            .collect()
    }
}
