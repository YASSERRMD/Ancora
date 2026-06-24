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
