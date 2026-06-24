package ancora

import (
	"encoding/json"
	"fmt"
	"reflect"
	"strings"
)

// jsonSchema is the internal representation used for schema generation.
type jsonSchema struct {
	Type        string                `json:"type"`
	Description string                `json:"description,omitempty"`
	Properties  map[string]jsonSchema `json:"properties,omitempty"`
	Required    []string              `json:"required,omitempty"`
}

// SchemaFromStruct generates a JSON Schema string from a Go struct value.
// It reads `json` tags for property names and `schema` tags for descriptions.
// Returns an error if v is not a struct or pointer to struct.
func SchemaFromStruct(v any) (string, error) {
	t := reflect.TypeOf(v)
	if t == nil {
		return "", fmt.Errorf("SchemaFromStruct: nil value")
	}
	if t.Kind() == reflect.Ptr {
		t = t.Elem()
	}
	if t.Kind() != reflect.Struct {
		return "", fmt.Errorf("SchemaFromStruct: expected struct, got %s", t.Kind())
	}

	schema := jsonSchema{
		Type:       "object",
		Properties: make(map[string]jsonSchema),
	}

	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		if !f.IsExported() {
			continue
		}

		name := f.Tag.Get("json")
		if name == "" || name == "-" {
			name = f.Name
		}
		if i := strings.IndexByte(name, ','); i >= 0 {
			name = name[:i]
		}

		prop := jsonSchema{
			Type:        kindToJSONType(f.Type.Kind()),
			Description: f.Tag.Get("schema"),
		}
		schema.Properties[name] = prop
		schema.Required = append(schema.Required, name)
	}

	b, err := json.Marshal(schema)
	if err != nil {
		return "", err
	}
	return string(b), nil
}

// kindToJSONType maps a Go reflect.Kind to a JSON Schema type string.
func kindToJSONType(k reflect.Kind) string {
	switch k {
	case reflect.String:
		return "string"
	case reflect.Bool:
		return "boolean"
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64,
		reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
		return "integer"
	case reflect.Float32, reflect.Float64:
		return "number"
	case reflect.Slice, reflect.Array:
		return "array"
	default:
		return "object"
	}
}
