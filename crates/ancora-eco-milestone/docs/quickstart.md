# Extension Author Quickstart

Get your first Ancora plugin published in 5 steps.

## Step 1: Install the Ancora CLI

```bash
cargo install ancora-cli
```

## Step 2: Scaffold a new plugin

```bash
ancora new-plugin my-plugin
```

## Step 3: Implement the plugin trait

Open `src/lib.rs` and implement the `Plugin` trait.

## Step 4: Build and test locally

```bash
cargo test
```

## Step 5: Publish to the registry

```bash
ancora publish
```
