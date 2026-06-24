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
