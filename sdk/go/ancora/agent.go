package ancora

// Agent is a high-level launcher that pairs a Runtime with an AgentSpec.
type Agent struct {
	rt   *Runtime
	spec *AgentSpec
}
