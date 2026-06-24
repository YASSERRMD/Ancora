package ancora

import "context"

// CgoTransport implements Transport via the FFI staticlib.
type CgoTransport struct {
	rt *Runtime
}

// NewCgoTransport wraps a Runtime in a CgoTransport.
func NewCgoTransport(rt *Runtime) *CgoTransport {
	return &CgoTransport{rt: rt}
}

// StartRun starts a new agent run using the FFI layer.
func (t *CgoTransport) StartRun(_ context.Context, spec []byte) (string, error) {
	run, err := t.rt.StartRun(spec)
	if err != nil {
		return "", err
	}
	return run.id, nil
}

// PollRun pops the next event for runID via the FFI layer.
func (t *CgoTransport) PollRun(_ context.Context, runID string) ([]byte, error) {
	b, code := cRunPoll(t.rt.ptr, runID)
	if err := asError(code); err != nil {
		return nil, err
	}
	return b, nil
}

// ResumeRun resumes a suspended run via the FFI layer.
func (t *CgoTransport) ResumeRun(_ context.Context, runID string, decision []byte) error {
	return asError(cRunResume(t.rt.ptr, runID, decision))
}
