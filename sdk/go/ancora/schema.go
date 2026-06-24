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
