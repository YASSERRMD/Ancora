use crate::error::StructuredError;
use crate::extractor::JsonExtractor;
use crate::schema::OutputSchema;
use crate::validator::OutputValidator;
use serde_json::Value;

pub struct RetryConfig {
    pub max_attempts: u32,
    pub include_error_in_prompt: bool,
}

impl RetryConfig {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            include_error_in_prompt: true,
        }
    }
}

pub struct StructuredRetry {
    config: RetryConfig,
}

impl StructuredRetry {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Try to get a valid structured response using the provided generator.
    /// `gen(attempt, last_error)` should call the model and return the response text.
    pub fn run<F>(&self, schema: &OutputSchema, mut gen: F) -> Result<Value, StructuredError>
    where
        F: FnMut(u32, Option<&str>) -> String,
    {
        let mut last_error: Option<String> = None;
        for attempt in 0..self.config.max_attempts {
            let text = gen(attempt, last_error.as_deref());
            match JsonExtractor::extract(&text) {
                Ok(value) => match OutputValidator::validate(schema, &value) {
                    Ok(()) => return Ok(value),
                    Err(e) => {
                        last_error = Some(e.to_string());
                    }
                },
                Err(e) => {
                    last_error = Some(e.to_string());
                }
            }
        }
        Err(StructuredError::RetryLimitReached {
            attempts: self.config.max_attempts,
        })
    }
}
