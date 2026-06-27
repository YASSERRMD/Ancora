# structured-output

Demonstrates how to derive a JSON Schema from a Go struct using
`SchemaFromStruct` and inject it into an agent system prompt so the agent
knows the expected output shape.

## Run

```bash
cd sdk/go
go run ./examples/structured-output
```

## What it shows

- Defining a struct with `json` and `schema` field tags
- Calling `ancora.SchemaFromStruct` to get a JSON Schema string
- Embedding the schema in the agent system prompt
- Starting the run and draining events
