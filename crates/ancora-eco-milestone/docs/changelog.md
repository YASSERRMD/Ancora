# Changelog

## v0.6.0 (2026-06-29)

### Added
- Ecosystem milestone: plugin catalog, registry, sample apps, ITK
- Plugin hot-reload support
- gRPC streaming transport

### Changed
- PluginCtx::invoke renamed to PluginCtx::call

### Fixed
- Catalog search pagination off-by-one

### Performance
- Registry fetch latency reduced by 40%

### Security
- Plugin sandbox escapes via symlinks patched
