# Extensibility Overview

Ancora provides four stable extension points for plugin authors:

- **plugin-sdk** - Implement the `Plugin` trait to add custom behaviour.
- **graph-builder** - Define custom node types in the task DAG.
- **cli-plugins** - Add subcommands to the `ancora` binary.
- **fw-adapters** - Bridge third-party orchestration frameworks (unstable).

See the individual module docs for each extension point.
