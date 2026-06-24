package ancora

import "google.golang.org/protobuf/proto"

// AgentSpecBuilder constructs an AgentSpec incrementally.
type AgentSpecBuilder struct {
	spec AgentSpec
}

// NewAgentSpecBuilder returns a builder with default values.
func NewAgentSpecBuilder() *AgentSpecBuilder {
	return &AgentSpecBuilder{}
}

// WithName sets the agent's stable machine-readable identifier.
func (b *AgentSpecBuilder) WithName(name string) *AgentSpecBuilder {
	b.spec.Name = name
	return b
}

// WithModelID sets the model identifier (e.g. "gpt-4o", "llama3").
func (b *AgentSpecBuilder) WithModelID(id string) *AgentSpecBuilder {
	b.spec.ModelId = id
	return b
}

// WithInstructions sets the system prompt sent before the conversation.
func (b *AgentSpecBuilder) WithInstructions(instructions string) *AgentSpecBuilder {
	b.spec.Instructions = instructions
	return b
}

// WithMaxSteps sets the maximum number of reason-act iterations.
func (b *AgentSpecBuilder) WithMaxSteps(n uint32) *AgentSpecBuilder {
	b.spec.MaxSteps = n
	return b
}

// WithTool appends a ToolSpec to the agent's tool list.
func (b *AgentSpecBuilder) WithTool(t *ToolSpec) *AgentSpecBuilder {
	b.spec.Tools = append(b.spec.Tools, t)
	return b
}

// Build returns the AgentSpec proto message.
func (b *AgentSpecBuilder) Build() *AgentSpec {
	spec := b.spec
	return &spec
}

// BuildBytes marshals the AgentSpec to proto-encoded bytes.
func (b *AgentSpecBuilder) BuildBytes() ([]byte, error) {
	return proto.Marshal(b.Build())
}

// ToolSpecBuilder constructs a ToolSpec incrementally.
type ToolSpecBuilder struct {
	spec ToolSpec
}

// NewToolSpecBuilder returns a builder for a ToolSpec.
func NewToolSpecBuilder() *ToolSpecBuilder {
	return &ToolSpecBuilder{}
}

// WithToolName sets the tool's stable machine-readable identifier.
func (b *ToolSpecBuilder) WithToolName(name string) *ToolSpecBuilder {
	b.spec.Name = name
	return b
}

// WithDescription sets the human-readable description shown to the model.
func (b *ToolSpecBuilder) WithDescription(desc string) *ToolSpecBuilder {
	b.spec.Description = desc
	return b
}

// WithInputSchema sets the JSON Schema string for the tool's input arguments.
func (b *ToolSpecBuilder) WithInputSchema(schema string) *ToolSpecBuilder {
	b.spec.InputSchemaJson = schema
	return b
}

// Build returns the ToolSpec proto message.
func (b *ToolSpecBuilder) Build() *ToolSpec {
	spec := b.spec
	return &spec
}
