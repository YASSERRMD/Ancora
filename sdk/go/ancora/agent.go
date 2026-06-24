package ancora

import "google.golang.org/protobuf/proto"

// Agent is a high-level launcher that pairs a Runtime with an AgentSpec.
type Agent struct {
	rt   *Runtime
	spec *AgentSpec
}

// NewAgent binds a runtime and an agent spec. Both must be non-nil.
func NewAgent(rt *Runtime, spec *AgentSpec) *Agent {
	return &Agent{rt: rt, spec: spec}
}

// Start serializes the agent spec and launches a new run.
func (a *Agent) Start() (*Run, error) {
	bytes, err := proto.Marshal(a.spec)
	if err != nil {
		return nil, err
	}
	return a.rt.StartRun(bytes)
}

// Resume provides a human-in-loop decision to a suspended run.
func (a *Agent) Resume(run *Run, decision []byte) error {
	return run.Resume(decision)
}
