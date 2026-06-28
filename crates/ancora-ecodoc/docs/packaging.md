# Packaging Templates

Follow this checklist before every plugin release.

## Mandatory steps

1. Bump the version in `Cargo.toml` (SemVer).
2. Add a `CHANGELOG.md` entry for the new version.
3. Run `cargo test` - all tests must pass.
4. Verify the crate compiles on the declared MSRV.
5. Update `ancora-catalog.toml` with the new version.
6. Create a git tag matching the crate version.

## Recommended steps

- Preview docs: `cargo doc --open`
- Dry-run publish: `cargo publish --dry-run`

## Packaging manifest

Each release should include an `ancora-catalog.toml` with the fields described in the Catalog Format doc.
