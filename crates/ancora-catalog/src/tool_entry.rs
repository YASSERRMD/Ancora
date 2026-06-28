use crate::entry_schema::EntrySchema;
use crate::metadata::Metadata;

/// Whether a tool has observable side effects outside the agent process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolEffect {
    /// Pure computation; no external state is modified.
    None,
    /// Reads from an external system but does not write.
    ReadOnly,
    /// May write to external systems.
    Write,
    /// May destroy or irreversibly alter external data.
    Destructive,
}

/// An entry in the catalog describing an installable tool.
#[derive(Debug, Clone)]
pub struct ToolEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub effect: ToolEffect,
    pub input_schema: EntrySchema,
    pub output_schema: EntrySchema,
    pub metadata: Metadata,
}

impl ToolEntry {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        effect: ToolEffect,
        metadata: Metadata,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            effect,
            input_schema: EntrySchema::new(),
            output_schema: EntrySchema::new(),
            metadata,
        }
    }

    pub fn with_input_schema(mut self, schema: EntrySchema) -> Self {
        self.input_schema = schema;
        self
    }

    pub fn with_output_schema(mut self, schema: EntrySchema) -> Self {
        self.output_schema = schema;
        self
    }

    /// A tool entry is considered valid when both its id and name are non-empty
    /// and both schemas are internally valid.
    pub fn is_valid(&self) -> bool {
        !self.id.is_empty()
            && !self.name.is_empty()
            && self.input_schema.is_valid()
            && self.output_schema.is_valid()
    }
}
