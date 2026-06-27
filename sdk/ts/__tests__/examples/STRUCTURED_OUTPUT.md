# structured-output

Demonstrates deriving a JSON Schema from a Zod model using `zodToInputSchema`
and injecting it into the agent system prompt so the agent produces
structured output.
Runs fully offline.

## Test

```bash
cd sdk/ts
npx jest __tests__/examples/structured-output-example
```

## What it shows

- Defining a Zod schema with `.describe()` field annotations
- Converting the schema to a JSON Schema object via `zodToInputSchema`
- Parsing and validating agent token output against the Zod schema
