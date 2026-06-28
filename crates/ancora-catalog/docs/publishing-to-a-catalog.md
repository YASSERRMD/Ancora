# Publishing to a Catalog

This document describes how to add new entries to an Ancora catalog and make
them discoverable by other projects.

## Steps

1. **Create the entry struct** using the appropriate constructor, for example:

   ```rust
   let entry = ToolEntry::new(
       "my-tool",
       "My Tool",
       "Does something useful.",
       ToolEffect::ReadOnly,
       Metadata::new(
           Version::new(1, 0, 0),
           Author::new("Alice").with_email("alice@example.com"),
           License::apache2(),
       )
       .with_tag("search")
       .with_repository("https://github.com/alice/my-tool"),
   );
   ```

2. **Validate the entry** before publishing:

   ```rust
   let errors = validation::validate_tool(&entry);
   if !errors.is_empty() {
       eprintln!("Validation failed: {:?}", errors);
       return;
   }
   ```

3. **Add the entry to a `CatalogIndex`**:

   ```rust
   let mut index = CatalogIndex::new();
   index.add_tool(entry);
   ```

4. **Sign the catalog** so consumers can verify its integrity:

   ```rust
   let key = SigningKey::new("publisher-key-1", secret_bytes);
   let payload: Vec<u8> = /* serialise index to TOML/JSON bytes */;
   let mut signed = SignedCatalog::new(payload);
   signed.sign(&key);
   ```

5. **Publish** the signed catalog payload and the public signing-key identifier
   to your catalog registry endpoint or file store.

## Guidelines

- Use stable, lower-kebab-case IDs. Once published, changing an ID is a
  breaking change.
- Increment the semver version on every release.
- Include at least one tag so users can find the entry via `search_by_tag`.
- Choose the narrowest `ToolEffect` variant that accurately describes the
  side effects of the tool.
