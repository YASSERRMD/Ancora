use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;
use std::collections::HashMap;

/// Build a data-extraction recipe.
pub fn build(params: &ParamSet) -> Recipe {
    let schema = params.get("schema").unwrap_or("default");
    let format = params.get("output_format").unwrap_or("json");

    let mut r = Recipe::new(
        "data-extraction",
        "Data Extraction",
        format!(
            "Extract structured data using schema '{}' and output as {}.",
            schema, format
        ),
    );

    r.add_step(RecipeStep::new(
        "preprocess",
        StepAction::Extract,
        "Normalize and clean raw input text",
    ));
    r.add_step(RecipeStep::new(
        "extract",
        StepAction::Extract,
        format!("Extract fields according to schema '{}'", schema),
    ));
    r.add_step(RecipeStep::new(
        "validate",
        StepAction::Review,
        "Validate extracted fields against schema constraints",
    ));
    r.add_step(RecipeStep::new(
        "format",
        StepAction::Generate,
        format!("Serialize output to {}", format),
    ));
    r
}

/// Extracted record mapping field names to values.
#[derive(Debug, Clone, Default)]
pub struct ExtractedRecord {
    pub fields: HashMap<String, String>,
}

impl ExtractedRecord {
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.fields.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(|s| s.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

/// Schema definition for extraction.
#[derive(Debug, Clone)]
pub struct Schema {
    pub fields: Vec<SchemaField>,
}

#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: String,
    pub required: bool,
}

impl Schema {
    pub fn new(fields: Vec<SchemaField>) -> Self {
        Self { fields }
    }

    /// Validate an extracted record against this schema.
    pub fn validate(&self, record: &ExtractedRecord) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for field in &self.fields {
            if field.required && record.get(&field.name).is_none() {
                errors.push(format!("required field '{}' is missing", field.name));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;

    #[test]
    fn build_recipe_has_four_steps() {
        let params = ParamSet::default();
        let r = build(&params);
        assert_eq!(r.step_count(), 4);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn schema_validation_catches_missing_required() {
        let schema = Schema::new(vec![SchemaField { name: "name".into(), required: true }]);
        let record = ExtractedRecord::default();
        assert!(schema.validate(&record).is_err());
    }

    #[test]
    fn schema_validation_passes_when_fields_present() {
        let schema = Schema::new(vec![SchemaField { name: "name".into(), required: true }]);
        let mut record = ExtractedRecord::default();
        record.set("name", "Alice");
        assert!(schema.validate(&record).is_ok());
    }
}
