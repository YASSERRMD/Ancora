# ABI Versioning Rules

The `include/ancora.h` file is the authoritative ABI snapshot.
Any change to the public C surface must be deliberate and reviewed.

## What constitutes an ABI break

- Removing or renaming an exported function
- Changing a function's parameter types or count
- Changing a struct's field names, types, or layout
- Reordering enum variants or changing their discriminant values
- Changing a `#[repr(C)]` struct size

## What is ABI-safe

- Adding new exported functions (additive)
- Adding new enum variants at the end with explicit discriminants
- Changing internal (non-`pub`) Rust implementation details
- Changing documentation comments

## Review process

Before merging a PR that changes `include/ancora.h`:

1. Run `cargo build -p ancora-ffi` to regenerate the snapshot.
2. Inspect the diff with `git diff crates/ancora-ffi/include/ancora.h`.
3. If the change is additive, label the PR `abi:additive`.
4. If the change is breaking, bump `ANCORA_ABI_VERSION` and label it `abi:breaking`.
5. Commit the updated snapshot alongside the Rust source change.
