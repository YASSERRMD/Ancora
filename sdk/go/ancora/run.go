package ancora

// Run is a handle to a live agent run, identified by its string ID.
type Run struct {
	rt *Runtime
	id string
}

// ID returns the unique run identifier.
func (r *Run) ID() string { return r.id }

// PollEvent pops the next event from the run's event queue.
// Returns nil, nil when no more events are available.
func (r *Run) PollEvent() ([]byte, error) {
	b, code := cRunPoll(r.rt.ptr, r.id)
	if err := asError(code); err != nil {
		return nil, err
	}
	return b, nil
}

// Resume provides a decision to a suspended run.
func (r *Run) Resume(decision []byte) error {
	return asError(cRunResume(r.rt.ptr, r.id, decision))
}
