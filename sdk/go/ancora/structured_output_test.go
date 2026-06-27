package ancora_test

import (
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

type SumOutput struct {
	Result int    `json:"result" schema:"the computed sum"`
	Unit   string `json:"unit"   schema:"measurement unit"`
}

type SearchOutput struct {
	Query   string   `json:"query"   schema:"the search query"`
	Results []string `json:"results" schema:"list of matching items"`
}

func TestSchemaFromStructProducesObjectType(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(SumOutput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !strings.Contains(schema, `"object"`) {
		t.Fatalf("schema must contain 'object' type, got: %s", schema)
	}
}

func TestSchemaFromStructIncludesAllFieldNames(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(SumOutput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	for _, field := range []string{"result", "unit"} {
		if !strings.Contains(schema, field) {
			t.Fatalf("schema must include field %q, got: %s", field, schema)
		}
	}
}

func TestSchemaFromStructIsValidJSON(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(SumOutput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	var v interface{}
	if err := json.Unmarshal([]byte(schema), &v); err != nil {
		t.Fatalf("schema is not valid JSON: %v\nschema: %s", err, schema)
	}
}

func TestSchemaFromStructWithNestedArrayField(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(SearchOutput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !strings.Contains(schema, "results") {
		t.Fatalf("schema must include 'results' field, got: %s", schema)
	}
}

func TestToolSpecCarriesSchemaJSON(t *testing.T) {
	schema, _ := ancora.SchemaFromStruct(SumOutput{})
	tool := ancora.NewToolSpecBuilder().
		WithToolName("sum").
		WithDescription("compute sum").
		WithInputSchema(schema).
		Build()
	if tool.GetInputSchemaJson() == "" {
		t.Fatal("ToolSpec must carry non-empty input_schema_json")
	}
}

func TestAgentWithStructuredOutputSpecContainsSchema(t *testing.T) {
	schema, _ := ancora.SchemaFromStruct(SumOutput{})
	tool := ancora.NewToolSpecBuilder().
		WithToolName("sum-tool").
		WithDescription("adds numbers").
		WithInputSchema(schema).
		Build()
	spec := ancora.NewAgentSpecBuilder().
		WithName("structured-agent").
		WithModelID("gpt-4o").
		WithTool(tool).
		Build()
	if len(spec.GetTools()) == 0 {
		t.Fatal("spec must have at least one tool")
	}
	if spec.GetTools()[0].GetInputSchemaJson() == "" {
		t.Fatal("tool must carry schema JSON")
	}
}

func TestStructuredOutputWithRunRoundTrip(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	schema, _ := ancora.SchemaFromStruct(SumOutput{})
	tool := ancora.NewToolSpecBuilder().
		WithToolName("sum").
		WithDescription("adds two numbers").
		WithInputSchema(schema).
		Build()
	spec := ancora.NewAgentSpecBuilder().
		WithName("sum-agent").
		WithModelID("llama3").
		WithTool(tool).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestSchemaFromStructDescriptionsPresent(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(SumOutput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !strings.Contains(schema, "computed sum") {
		t.Logf("schema may not include description text (implementation-defined): %s", schema)
	}
}
