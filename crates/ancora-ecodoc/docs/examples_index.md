# Extension Examples Index

The following examples are shipped with the Ancora repository.

| Name            | Category          | Description                                  |
|-----------------|-------------------|----------------------------------------------|
| hello-plugin    | plugin            | Minimal plugin that logs lifecycle events    |
| counter-node    | graph-node        | Graph node that counts processed items       |
| greet-cli       | cli-command       | CLI subcommand that greets the user          |
| stub-adapter    | framework-adapter | No-op framework adapter for testing          |
| copy-recipe     | recipe            | Workflow recipe that copies a file           |

Each example lives under `examples/<name>/` in the repository root and can be built with `cargo build --example <name>`.
