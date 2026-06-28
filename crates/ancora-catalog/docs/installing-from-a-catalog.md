# Installing from a Catalog

This document describes how to discover and install entries from an Ancora
catalog into your project.

## Steps

1. **Load the catalog index** (from a local file, registry, or remote source):

   ```rust
   let index: CatalogIndex = /* load from bytes */;
   ```

2. **Verify the signature** (recommended):

   ```rust
   let key = SigningKey::new("publisher-key-1", known_public_bytes);
   let signed = SignedCatalog { payload, signature: Some(sig) };
   if !signed.verify(&key) {
       return Err("catalog signature verification failed");
   }
   ```

3. **Search for entries** by tag or name:

   ```rust
   let hits = search::search_by_tag(&index, "search");
   for hit in &hits {
       println!("{}: {} ({})", hit.kind, hit.name, hit.id);
   }
   ```

4. **Install an entry** into your project registry:

   ```rust
   let mut registry = ProjectRegistry::new();
   if let Some(tool) = index.find_tool("web-search") {
       registry.install_tool(tool)?;
   }
   ```

5. **Verify the license** of every installed entry:

   ```rust
   for entry in &registry.entries {
       println!("{} is licensed under {}", entry.name, entry.license.as_str());
   }
   ```

## Offline usage

The catalog and `ProjectRegistry` are entirely in-memory and require no
network access. Load catalog bytes from a local file and install offline.

## Idempotency

Installing the same entry twice returns an `InstallError` with the message
"already installed". Check `registry.is_installed(id)` before installing if
you want idempotent behaviour.
