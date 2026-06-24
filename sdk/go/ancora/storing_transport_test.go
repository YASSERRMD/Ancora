package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestStoringTransportImplementsTransport(t *testing.T) {
	s := mustInMemoryStore(t)
	tr := newFakeGRPCTransport(t)
	var _ ancora.Transport = ancora.NewStoringTransport(tr, s)
}

func TestStoringTransportStartRunRecordsRunID(t *testing.T) {
	s := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(newFakeGRPCTransport(t), s)
	id, err := tr.StartRun(context.Background(), []byte("spec"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	ok, err := s.HasRun(id)
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if !ok {
		t.Fatalf("run %q not recorded in store", id)
	}
}

func TestStoringTransportPollRunStoresEvent(t *testing.T) {
	s := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(newEventGRPCTransport(t, []string{"ev1"}), s)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	_, err := tr.PollRun(context.Background(), id)
	if err != nil {
		t.Fatalf("PollRun: %v", err)
	}
	evs, err := s.EventsForRun(id)
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(evs) != 1 || evs[0] != "ev1" {
		t.Fatalf("expected [ev1], got: %v", evs)
	}
}

func TestStoringTransportPollRunNilEventNotStored(t *testing.T) {
	s := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(newFakeGRPCTransport(t), s)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	tr.PollRun(context.Background(), id)
	evs, _ := s.EventsForRun(id)
	if len(evs) != 0 {
		t.Fatalf("expected 0 stored events for empty poll, got: %d", len(evs))
	}
}

func TestStoringTransportResumeRunSucceeds(t *testing.T) {
	s := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(newFakeGRPCTransport(t), s)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	err := tr.ResumeRun(context.Background(), id, []byte(`{}`))
	if err != nil {
		t.Fatalf("ResumeRun: %v", err)
	}
}

func TestStoringTransportMultipleEventsStoredInOrder(t *testing.T) {
	s := mustInMemoryStore(t)
	events := []string{"e1", "e2", "e3"}
	tr := ancora.NewStoringTransport(newEventGRPCTransport(t, events), s)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	for i := 0; i < 3; i++ {
		tr.PollRun(context.Background(), id)
	}
	evs, _ := s.EventsForRun(id)
	if len(evs) != 3 || evs[0] != "e1" || evs[2] != "e3" {
		t.Fatalf("events not stored in order: %v", evs)
	}
}

func TestStoringTransportCgoBackendPersistsRunID(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	s := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), s)
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("store-agent").WithModelID("llama3").WithInstructions("hi").
		BuildBytes()
	id, err := tr.StartRun(context.Background(), b)
	if err != nil {
		t.Fatalf("StartRun via cgo: %v", err)
	}
	ok, _ := s.HasRun(id)
	if !ok {
		t.Fatalf("run %q not persisted via cgo StoringTransport", id)
	}
}
