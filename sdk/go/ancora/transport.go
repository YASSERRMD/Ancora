package ancora

import "context"

// Transport abstracts the run lifecycle over cgo or gRPC.
type Transport interface {
	// StartRun serializes spec bytes and returns a run ID.
	StartRun(ctx context.Context, spec []byte) (string, error)
	// PollRun pops the next event for runID. Returns nil bytes when empty.
	PollRun(ctx context.Context, runID string) ([]byte, error)
	// ResumeRun provides a human decision to a suspended run.
	ResumeRun(ctx context.Context, runID string, decision []byte) error
}
