# Catalog Format Specification

The Ancora catalog format describes a versioned, signable index of installable
components: tools, connectors, providers, and vector stores.

## Entry kinds

| Kind | Struct | Description |
|---|---|---|
| `tool` | `ToolEntry` | A callable function exposed to the agent. |
| `connector` | `ConnectorEntry` | An MCP server the agent can connect to. |
| `provider` | `ProviderEntry` | An LLM or embedding API provider. |
| `vector_store` | `VectorStoreEntry` | A vector database backend. |

## Required fields

Every entry must have:

- `id` - unique string identifier within the catalog (no spaces)
- `name` - human-readable display name
- `description` - short prose description
- `metadata.version` - semver triple (major, minor, patch)
- `metadata.author.name` - name of the person or organisation that published the entry
- `metadata.license` - SPDX license identifier (e.g. `Apache-2.0`, `MIT`)

## Optional fields

- `metadata.tags` - list of searchable strings
- `metadata.homepage` - canonical URL for the component
- `metadata.repository` - source code repository URL

## Schema fields

Tool entries carry an `input_schema` and an `output_schema`. Each schema is a
list of `SchemaField` records:

```
SchemaField {
    name: String,        // non-empty
    ty: FieldType,       // String | Integer | Float | Boolean | Array(T) | Object
    required: bool,
    description: Option<String>,
}
```

## Index layout

A `CatalogIndex` aggregates all entries. Entries within each category are
stored in insertion order. IDs must be unique within a category but may
collide across categories.

## Signing

A catalog payload (the canonical serialised bytes of the index) may be signed
with a `SigningKey`. The signature is stored alongside the payload in a
`SignedCatalog`. Verification uses the same key and checks that the HMAC over
the payload matches the stored value.
