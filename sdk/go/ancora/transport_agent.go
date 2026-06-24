package ancora

import (
	"context"

	"google.golang.org/protobuf/proto"
)

// TransportRun is a handle to a live agent run backed by a Transport.
type TransportRun struct {
	tr    Transport
	runID string
}

// ID returns the unique run identifier.
func (r *TransportRun) ID() string { return r.runID }

// PollEvent pops the next event for this run via the Transport.
// Returns nil, nil when no event is available.
func (r *TransportRun) PollEvent(ctx context.Context) ([]byte, error) {
	return r.tr.PollRun(ctx, r.runID)
}

// Resume provides a decision to a suspended run via the Transport.
func (r *TransportRun) Resume(ctx context.Context, decision []byte) error {
	return r.tr.ResumeRun(ctx, r.runID, decision)
}

// transportRunChanBuf is the buffer size for TransportRun.EventChan channels.
const transportRunChanBuf = 64

// DrainEvents collects all pending events from the run into a string slice.
func (r *TransportRun) DrainEvents(ctx context.Context) ([]string, error) {
	var out []string
	for {
		ev, err := r.PollEvent(ctx)
		if err != nil {
			return out, err
		}
		if ev == nil {
			return out, nil
		}
		out = append(out, string(ev))
	}
}

// EventChan spawns a goroutine polling events and returns a channel.
// The channel is closed when the event queue is empty or ctx is done.
func (r *TransportRun) EventChan(ctx context.Context) <-chan []byte {
	ch := make(chan []byte, transportRunChanBuf)
	go func() {
		defer close(ch)
		for {
			ev, err := r.PollEvent(ctx)
			if err != nil || ev == nil {
				return
			}
			select {
			case ch <- ev:
			case <-ctx.Done():
				return
			}
		}
	}()
	return ch
}

// TransportAgent starts agent runs through a Transport.
type TransportAgent struct {
	tr   Transport
	spec *AgentSpec
}

// NewTransportAgent returns an agent that uses tr to drive the run lifecycle.
func NewTransportAgent(tr Transport, spec *AgentSpec) *TransportAgent {
	return &TransportAgent{tr: tr, spec: spec}
}

// Start serializes the AgentSpec and starts a run through the Transport.
func (a *TransportAgent) Start(ctx context.Context) (*TransportRun, error) {
	b, err := proto.Marshal(a.spec)
	if err != nil {
		return nil, err
	}
	runID, err := a.tr.StartRun(ctx, b)
	if err != nil {
		return nil, err
	}
	return &TransportRun{tr: a.tr, runID: runID}, nil
}

// Spec returns the AgentSpec used by this agent.
func (a *TransportAgent) Spec() *AgentSpec { return a.spec }

// Transport returns the underlying Transport.
func (a *TransportAgent) Transport() Transport { return a.tr }
