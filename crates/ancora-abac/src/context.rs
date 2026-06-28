use crate::attribute::AttributeSet;

#[derive(Debug, Default)]
pub struct RequestContext {
    pub subject: AttributeSet,
    pub resource: AttributeSet,
    pub environment: AttributeSet,
}

impl RequestContext {
    pub fn new() -> Self { Self::default() }

    pub fn with_subject_attr(mut self, key: impl Into<String>, value: impl Into<crate::attribute::AttributeValue>) -> Self {
        self.subject.set(key, value);
        self
    }

    pub fn with_resource_attr(mut self, key: impl Into<String>, value: impl Into<crate::attribute::AttributeValue>) -> Self {
        self.resource.set(key, value);
        self
    }

    pub fn with_env_attr(mut self, key: impl Into<String>, value: impl Into<crate::attribute::AttributeValue>) -> Self {
        self.environment.set(key, value);
        self
    }
}
