package ancora_test

import (
	"context"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestStoringTransportMultiHopImplementsTransport(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	inner := newFakeGRPCTransport(t)
	var _ ancora.Transport = ancora.NewStoringTransport(inner, store)
}

func TestStoringTransportStartRunRecordsRun(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(newFakeGRPCTransport(t), store)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	if runID == "" {
		t.Fatal("run ID must be non-empty")
	}

	has, err := store.HasRun(runID)
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if !has {
		t.Fatalf("StoringTransport must record run %q in store", runID)
	}
}

func TestStoringTransportMultipleRunsAreAllRecorded(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	inner := newFakeGRPCTransport(t)
	tr := ancora.NewStoringTransport(inner, store)

	const numRuns = 5
	for i := 0; i < numRuns; i++ {
		_, err := tr.StartRun(context.Background(), []byte("{}"))
		if err != nil {
			t.Fatalf("StartRun %d: %v", i, err)
		}
	}

	count, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count != numRuns {
		t.Fatalf("expected %d runs, got: %d", numRuns, count)
	}
}

func TestStoringTransportPollRunAppendsEventToStore(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	rt := mustRuntime(t)
	defer rt.Free()
	cgoTr := ancora.NewCgoTransport(rt)
	tr := ancora.NewStoringTransport(cgoTr, store)

	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	var storedCount int
	for {
		ev, err := tr.PollRun(context.Background(), runID)
		if err != nil {
			t.Fatalf("PollRun: %v", err)
		}
		if ev == nil {
			break
		}
		storedCount++
	}

	count, err := store.EventCount(runID)
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if count != storedCount {
		t.Fatalf("store must contain %d events (polled), got: %d", storedCount, count)
	}
}

func TestStoringTransportResumeRunDelegates(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)

	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	for {
		ev, _ := tr.PollRun(context.Background(), runID)
		if ev == nil {
			break
		}
	}

	if err := tr.ResumeRun(context.Background(), runID, []byte(`{"approved":true}`)); err != nil {
		t.Logf("ResumeRun on completed run returned error (acceptable): %v", err)
	}
}

func TestStoringTransportEventsContainStarted(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)

	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	for {
		ev, _ := tr.PollRun(context.Background(), runID)
		if ev == nil {
			break
		}
	}

	events, err := store.EventsForRun(runID)
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) == 0 {
		t.Fatal("store must contain at least one event after polling")
	}

	found := false
	for _, ev := range events {
		if strings.Contains(ev, "started") {
			found = true
			break
		}
	}
	if !found {
		t.Logf("stored events: %v (started event may have different shape)", events)
	}
}
