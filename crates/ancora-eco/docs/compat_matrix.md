# Compatibility Matrix

The Ancora extension ecosystem publishes a compatibility matrix with each
core release. The matrix records the negotiation result for every registered
extension against the new core version.

## Format

| Extension ID | Ext Min API | Ext Max API | Core Version | Status |
|---|---|---|---|---|
| my-ext | 1.0.0 | 1.9.0 | 1.5.0 | Compatible |
| old-ext | 1.0.0 | 1.2.0 | 1.5.0 | ExtensionTooOld |
| v2-ext | 2.0.0 | 2.5.0 | 1.5.0 | MajorMismatch |

## Generation

The matrix is generated automatically by the CI pipeline using the
`CompatMatrix` type from `ancora-eco::compat_matrix`. The `generate_report()`
method produces a human-readable text summary; JSON output is also available
for machine consumption.

## Policy

Extensions in the `ExtensionTooOld` or `MajorMismatch` state are listed but
grayed out in the extension registry. Extensions in `Compatible` state are
fully listed.

## Publication

The matrix is published at `https://registry.ancora-project.example/compat`
after each core release and is also bundled as an artifact in the release
GitHub Actions workflow.
