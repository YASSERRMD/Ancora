# Workflow Recipes

Recipes are pre-built workflow templates that you can instantiate with parameters.

## Built-in recipes

| Name            | Description                                        |
|-----------------|----------------------------------------------------|
| `http-fetch`    | Fetch a URL and pipe the response body downstream  |
| `file-transform`| Read a file, transform it, and write the result    |
| `parallel-fanout`| Fan a single input to N parallel branches         |

## Using a recipe

```rust
use ancora_ecodoc::recipes::{find_recipe};
use std::collections::HashMap;

let recipe = find_recipe("http-fetch").unwrap();
let mut params = HashMap::new();
params.insert("url", "https://api.example.com/data");
recipe.validate(&params)?;
```

## Writing a custom recipe

Recipes are plain Rust structs. Implement `Recipe` and add your struct to the built-in list or register it at runtime.
