# Authoring Recipes

A recipe is a `format::Recipe` value built from `RecipeStep` items.

## Minimal recipe

```rust
use ancora_recipe::format::{Recipe, RecipeStep, StepAction};

let mut r = Recipe::new("my-recipe", "My Recipe", "A description.");
r.add_step(RecipeStep::new("fetch", StepAction::Retrieve, "Fetch relevant data"));
r.add_step(RecipeStep::new("generate", StepAction::Generate, "Generate output"));
assert!(r.validate().is_ok());
```

## Accepting parameters

Recipes should accept a `&ParamSet` and read values with `.get(key).unwrap_or(default)`:

```rust
use ancora_recipe::params::ParamSet;

pub fn build(params: &ParamSet) -> Recipe {
    let n: usize = params.get("n").and_then(|v| v.parse().ok()).unwrap_or(3);
    // use n to configure steps ...
}
```

## Step actions

Choose from the predefined `StepAction` variants:

- `Retrieve` - fetch data from a source
- `Generate` - produce text or structured output
- `Review` - evaluate or critique content
- `Extract` - parse or decompose input
- `Classify` - assign a category to content
- `Summarize` - condense content
- `Debate` - structured multi-agent argument
- `Install` - install an artifact
- `Custom(String)` - any other action

## Validation rules

- `id` must be non-empty.
- At least one step must be present.

## Testing your recipe

Write a `#[test]` that calls your `build` function with a `ParamSet::default()` and asserts that `validate()` returns `Ok(())`.
All recipe tests must run offline with no network or filesystem I/O.
