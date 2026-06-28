# Semantic Versioning for Extension Points

All extension API points in Ancora follow Semantic Versioning 2.0.0.

## Version Format

`MAJOR.MINOR.PATCH`

- **MAJOR**: Incremented for incompatible API changes. Extensions must declare
  compatibility with a specific major version.
- **MINOR**: Incremented for backward-compatible new features. Extensions
  declaring `min_api_version = 1.2.0` will load on any core `>= 1.2.0, < 2.0.0`.
- **PATCH**: Incremented for backward-compatible bug fixes. No impact on
  extension compatibility ranges.

## Extension Manifests

Each extension declares:

```
min_api_version = "1.2.0"
max_api_version = "1.9.0"
```

The core negotiates at load time whether the running API version falls within
the extension's declared range.

## Breaking Change Detection

The `SemVer::is_breaking_bump` function detects whether a version transition
constitutes a breaking change (major version increase).
