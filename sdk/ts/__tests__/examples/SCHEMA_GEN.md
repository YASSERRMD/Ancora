# schema-gen

Demonstrates the `zodToInputSchema` helper that converts a Zod object schema
into a JSON Schema compatible with Ancora `ToolSpec.input_schema`.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/schema-gen-example
```

## What it shows

- Converting Zod field types (string, number, boolean, array) to JSON Schema
- Preserving `.describe()` annotations as the `description` property
- Marking optional fields as non-required in the output schema
