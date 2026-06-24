package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

type searchInput struct {
	Query string `json:"query" schema:"search query string"`
	Limit int    `json:"limit" schema:"max number of results"`
}

type emptyStruct struct{}

func TestSchemaFromStructHasObjectType(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, `"type":"object"`) {
		t.Fatalf("schema must have object type, got: %s", schema)
	}
}

func TestSchemaFromStructIncludesJSONFieldNames(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, `"query"`) {
		t.Fatalf("schema missing 'query' field, got: %s", schema)
	}
	if !contains(schema, `"limit"`) {
		t.Fatalf("schema missing 'limit' field, got: %s", schema)
	}
}

func TestSchemaFromStructIncludesDescriptions(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, "search query string") {
		t.Fatalf("schema missing description from schema tag, got: %s", schema)
	}
}

func TestSchemaFromStructStringFieldHasStringType(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, `"string"`) {
		t.Fatalf("schema missing string type, got: %s", schema)
	}
}

func TestSchemaFromStructIntFieldHasIntegerType(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, `"integer"`) {
		t.Fatalf("schema missing integer type, got: %s", schema)
	}
}

func TestSchemaFromStructNonStructReturnsError(t *testing.T) {
	_, err := ancora.SchemaFromStruct("not a struct")
	if err == nil {
		t.Fatal("SchemaFromStruct with string should return error")
	}
}

func TestSchemaFromStructPointerToStruct(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(&searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct(*struct): %v", err)
	}
	if !contains(schema, `"type":"object"`) {
		t.Fatalf("pointer-to-struct schema must have object type, got: %s", schema)
	}
}

func TestSchemaFromStructEmptyStructHasNoProperties(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(emptyStruct{})
	if err != nil {
		t.Fatalf("SchemaFromStruct(emptyStruct): %v", err)
	}
	if !contains(schema, `"type":"object"`) {
		t.Fatalf("empty struct must still be object type, got: %s", schema)
	}
}

func TestSchemaFromStructBoolFieldHasBooleanType(t *testing.T) {
	type boolStruct struct {
		Active bool `json:"active"`
	}
	schema, err := ancora.SchemaFromStruct(boolStruct{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, `"boolean"`) {
		t.Fatalf("bool field must map to boolean type, got: %s", schema)
	}
}

func TestSchemaFromStructRequiredListContainsFields(t *testing.T) {
	schema, err := ancora.SchemaFromStruct(searchInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if !contains(schema, `"required"`) {
		t.Fatalf("schema must include required list, got: %s", schema)
	}
}
