/// The top-level kind of a catalog entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Tool,
    Connector,
    Provider,
    VectorStore,
}

/// A JSON-Schema-like description of one field in an entry's input/output schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaField {
    pub name: String,
    pub ty: FieldType,
    pub required: bool,
    pub description: Option<String>,
}

/// Primitive field types supported by the catalog schema language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Array(Box<FieldType>),
    Object,
}

/// The input/output schema attached to a catalog entry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EntrySchema {
    pub fields: Vec<SchemaField>,
}

impl EntrySchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field(mut self, field: SchemaField) -> Self {
        self.fields.push(field);
        self
    }

    /// Return all required fields.
    pub fn required_fields(&self) -> Vec<&SchemaField> {
        self.fields.iter().filter(|f| f.required).collect()
    }

    /// Check that all required fields have non-empty names.
    pub fn is_valid(&self) -> bool {
        self.fields.iter().all(|f| !f.name.is_empty())
    }
}

impl SchemaField {
    pub fn new(name: impl Into<String>, ty: FieldType, required: bool) -> Self {
        Self {
            name: name.into(),
            ty,
            required,
            description: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}
