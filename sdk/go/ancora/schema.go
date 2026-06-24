package ancora

import (
	"encoding/json"
	"fmt"
	"reflect"
)

// jsonSchema is the internal representation used for schema generation.
type jsonSchema struct {
	Type        string                `json:"type"`
	Description string                `json:"description,omitempty"`
	Properties  map[string]jsonSchema `json:"properties,omitempty"`
	Required    []string              `json:"required,omitempty"`
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
