package ancora

// NewCgoTransportAgent creates a TransportAgent backed by an in-process
// CgoTransport wrapping rt. It is a convenience over NewTransportAgent.
func NewCgoTransportAgent(rt *Runtime, spec *AgentSpec) *TransportAgent {
	return NewTransportAgent(NewCgoTransport(rt), spec)
}
