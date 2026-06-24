# ABI Versioning Rules

The `include/ancora.h` file is the authoritative ABI snapshot.
Any change to the public C surface must be deliberate and reviewed.

## What constitutes an ABI break

- Removing or renaming an exported function
- Changing a function's parameter types or count
- Changing a struct's field names, types, or layout
- Reordering enum variants or changing their discriminant values
- Changing a `#[repr(C)]` struct size
