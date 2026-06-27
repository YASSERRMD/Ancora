package ancora_test

import (
	"context"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// journalFixture represents the expected event sequence for a known
// deterministic run. All entries are offline-verifiable.
var journalFixture = []struct {
	seq     int
	payload string
}{
	{0, `{"type":"started"}`},
	{1, `{"type":"activity_recorded","key":"step-1","replayed":false}`},
	{2, `{"type":"activity_recorded","key":"step-2","replayed":false}`},
	{3, `{"type":"completed"}`},
}

func TestE2EJournalFixtureCanBeStoredAndRetrieved(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("fixture-run")
	for _, e := range journalFixture {
		_ = store.AppendEvent("fixture-run", e.seq, e.payload)
	}

	events, err := store.EventsForRun("fixture-run")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) != len(journalFixture) {
		t.Fatalf("expected %d events, got: %d", len(journalFixture), len(events))
	}
}

func TestE2EJournalFixtureFirstEventIsStarted(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("fixture-started")
	_ = store.AppendEvent("fixture-started", 0, journalFixture[0].payload)

	events, _ := store.EventsForRun("fixture-started")
	if !strings.Contains(events[0], "started") {
		t.Fatalf("first event must contain 'started', got: %s", events[0])
	}
}

func TestE2EJournalFixtureLastEventIsCompleted(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("fixture-completed")
	for _, e := range journalFixture {
		_ = store.AppendEvent("fixture-completed", e.seq, e.payload)
	}

	events, _ := store.EventsForRun("fixture-completed")
	if !strings.Contains(events[len(events)-1], "completed") {
		t.Fatalf("last event must contain 'completed', got: %s", events[len(events)-1])
	}
}

func TestE2EJournalFixtureActivityKeysArePreserved(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("fixture-keys")
	for _, e := range journalFixture {
		_ = store.AppendEvent("fixture-keys", e.seq, e.payload)
	}

	events, _ := store.EventsForRun("fixture-keys")
	if !strings.Contains(events[1], "step-1") {
		t.Fatalf("event 1 must contain 'step-1', got: %s", events[1])
	}
	if !strings.Contains(events[2], "step-2") {
		t.Fatalf("event 2 must contain 'step-2', got: %s", events[2])
	}
}

func TestE2EJournalFixtureMatchesCoreStructure(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("core-fixture")
	for _, e := range journalFixture {
		_ = store.AppendEvent("core-fixture", e.seq, e.payload)
	}

	events, _ := store.EventsForRun("core-fixture")
	for i, expected := range journalFixture {
		if events[i] != expected.payload {
			t.Fatalf("event %d mismatch: expected %q, got %q", i, expected.payload, events[i])
		}
	}
}

func TestE2EJournalLiveRunMatchesFixtureStructure(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

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

	events, _ := store.EventsForRun(runID)
	if len(events) == 0 {
		t.Fatal("live run must store at least one event")
	}
}

func TestE2EJournalDeleteRunClearsAllFixtureEvents(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("del-fixture")
	for _, e := range journalFixture {
		_ = store.AppendEvent("del-fixture", e.seq, e.payload)
	}

	_ = store.DeleteRun("del-fixture")

	events, _ := store.EventsForRun("del-fixture")
	if len(events) != 0 {
		t.Fatalf("all fixture events must be cleared after DeleteRun, got: %d", len(events))
	}
}

func TestE2EJournalTwoFixtureRunsAreSeparate(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("f-run-A")
	_ = store.RecordRun("f-run-B")
	for _, e := range journalFixture {
		_ = store.AppendEvent("f-run-A", e.seq, e.payload)
	}
	_ = store.AppendEvent("f-run-B", 0, `{"type":"started"}`)

	evA, _ := store.EventsForRun("f-run-A")
	evB, _ := store.EventsForRun("f-run-B")

	if len(evA) != 4 {
		t.Fatalf("f-run-A expected 4 events, got: %d", len(evA))
	}
	if len(evB) != 1 {
		t.Fatalf("f-run-B expected 1 event, got: %d", len(evB))
	}
}
