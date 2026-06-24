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

// eventChanBuf is the buffer size for EventChan channels.
const eventChanBuf = 64

// DrainEvents collects all pending events into a string slice.
func (r *Run) DrainEvents() ([]string, error) {
	var out []string
	for {
		ev, err := r.PollEvent()
		if err != nil {
			return out, err
		}
		if ev == nil {
			return out, nil
		}
		out = append(out, string(ev))
	}
}

// EventChan spawns a goroutine that polls events and sends them on a channel.
// The channel is closed when the event queue is empty.
func (r *Run) EventChan() <-chan []byte {
	ch := make(chan []byte, eventChanBuf)
	go func() {
		defer close(ch)
		for {
			ev, err := r.PollEvent()
			if err != nil || ev == nil {
				return
			}
			ch <- ev
		}
	}()
	return ch
}
