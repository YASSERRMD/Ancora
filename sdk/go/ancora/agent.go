package ancora

// Agent is a high-level launcher that pairs a Runtime with an AgentSpec.
type Agent struct {
	rt   *Runtime
	spec *AgentSpec
}

// NewAgent binds a runtime and an agent spec. Both must be non-nil.
func NewAgent(rt *Runtime, spec *AgentSpec) *Agent {
	return &Agent{rt: rt, spec: spec}
}
