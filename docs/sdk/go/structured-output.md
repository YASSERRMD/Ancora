# Typed Structured Output

Use `SchemaFromStruct` to derive a JSON Schema from a Go struct and request
typed output from the model.

## Define a result type

```go
type AnalysisResult struct {
    Summary   string  `json:"summary"`
    Sentiment string  `json:"sentiment"`
    Score     float64 `json:"score"`
}
```

## Attach the schema to the spec

```go
schema := ancora.SchemaFromStruct(AnalysisResult{})

spec := ancora.NewAgentSpec("llama3", "Analyze the text and return JSON.")
spec.OutputSchema = schema
```

## Deserialise the output

```go
events, _ := run.CollectAll()
last := events[len(events)-1].(*ancora.CompletedEvent)
var result AnalysisResult
json.Unmarshal([]byte(last.OutputJSON), &result)
fmt.Println(result.Sentiment) // "positive"
```

## Multiple schemas

You can define multiple struct types and select between them at runtime:

```go
schemas := map[string]ancora.Schema{
    "analysis":       ancora.SchemaFromStruct(AnalysisResult{}),
    "classification": ancora.SchemaFromStruct(ClassificationResult{}),
}
spec.OutputSchema = schemas[mode]
```

## See also

- [Quickstart](quickstart.md)
- [API reference](api-reference.md)
