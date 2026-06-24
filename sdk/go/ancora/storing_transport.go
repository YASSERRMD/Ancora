package ancora

import "context"

// StoringTransport wraps a Transport and records run IDs and events
// to a SqliteStore after each operation.
type StoringTransport struct {
	inner Transport
	store *SqliteStore
}

// NewStoringTransport returns a StoringTransport that delegates to inner
// and persists run metadata to store.
func NewStoringTransport(inner Transport, store *SqliteStore) *StoringTransport {
	return &StoringTransport{inner: inner, store: store}
}

// StartRun starts a run via the inner Transport and records the run ID.
func (t *StoringTransport) StartRun(ctx context.Context, spec []byte) (string, error) {
	id, err := t.inner.StartRun(ctx, spec)
	if err != nil {
		return "", err
	}
	_ = t.store.RecordRun(id)
	return id, nil
}

// PollRun polls the inner Transport and, when an event is available,
// appends it to the store.
func (t *StoringTransport) PollRun(ctx context.Context, runID string) ([]byte, error) {
	ev, err := t.inner.PollRun(ctx, runID)
	if err != nil || ev == nil {
		return ev, err
	}
	evs, _ := t.store.EventsForRun(runID)
	_ = t.store.AppendEvent(runID, len(evs), string(ev))
	return ev, nil
}

// ResumeRun resumes via the inner Transport (no store side-effect).
func (t *StoringTransport) ResumeRun(ctx context.Context, runID string, decision []byte) error {
	return t.inner.ResumeRun(ctx, runID, decision)
}
