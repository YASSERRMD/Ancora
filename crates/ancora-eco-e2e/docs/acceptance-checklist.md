# Acceptance Checklist

Phase 256 - Ecosystem E2E

## Plugin Lifecycle

- [ ] Plugin can be authored from a template
- [ ] Plugin passes the interop kit
- [ ] Plugin can be published to a local registry
- [ ] Plugin can be installed from a registry
- [ ] Plugin runs sandboxed
- [ ] Plugin crash is isolated from siblings

## Catalog and Recipes

- [ ] Catalog install adds a tool and it is discoverable
- [ ] Recipe installs and all steps execute
- [ ] Recipe uninstall removes it from the runner

## Graph Builder

- [ ] Graph builder accepts nodes and edges
- [ ] Topological ordering is correct for a DAG
- [ ] Cycle detection works correctly

## CLI Plugin

- [ ] CLI plugin can register commands
- [ ] CLI plugin dispatches commands to handlers
- [ ] Unknown commands return an error

## Framework Adapter

- [ ] Adapter imports a tool from a plugin
- [ ] Adapter lists tools sorted by name
- [ ] Duplicate imports are rejected

## Trust Policy

- [ ] Trust gate blocks plugins below minimum level
- [ ] Trust gate requires verified publisher unless overridden
- [ ] Checksum is required unless policy opts out
- [ ] Official trust level satisfies strict policy

## Air-gap

- [ ] Plugins can be bundled offline
- [ ] Bundle verifies checksums
- [ ] Air-gapped registry installs from bundle only
- [ ] Missing plugin returns an error

## Parity

- [ ] Rust and Python extensions have full capability parity
- [ ] All default runtimes score 1.0 parity
- [ ] Partial parity is calculated correctly

## CI

- [ ] All tests pass in CI without network access
- [ ] Build completes under 2 minutes
- [ ] No external crate dependencies
