package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestJournalReplayStoreRecordsRunAndEvents(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("replay-run-1")
	for i, ev := range []string{
		`{"type":"started","run_id":"replay-run-1"}`,
		`{"type":"activity_recorded","key":"step-1","replayed":false}`,
		`{"type":"activity_recorded","key":"step-2","replayed":false}`,
		`{"type":"completed"}`,
	} {
		_ = store.AppendEvent("replay-run-1", i, ev)
	}

	events, err := store.EventsForRun("replay-run-1")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) != 4 {
		t.Fatalf("expected 4 journal events, got: %d", len(events))
	}
}

func TestJournalReplayEventsContainActivityKeys(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("replay-keys")
	_ = store.AppendEvent("replay-keys", 0, `{"type":"activity_recorded","key":"act-abc","replayed":false}`)
	_ = store.AppendEvent("replay-keys", 1, `{"type":"activity_recorded","key":"act-def","replayed":true}`)

	events, _ := store.EventsForRun("replay-keys")
	if !strings.Contains(events[0], "act-abc") {
		t.Fatalf("first event must contain key 'act-abc', got: %s", events[0])
	}
	if !strings.Contains(events[1], "act-def") {
		t.Fatalf("second event must contain key 'act-def', got: %s", events[1])
	}
}

func TestJournalReplayReplayedFlagIsPreserved(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("replay-flag")
	_ = store.AppendEvent("replay-flag", 0, `{"type":"activity_recorded","key":"k1","replayed":true}`)

	events, _ := store.EventsForRun("replay-flag")
	if !strings.Contains(events[0], `"replayed":true`) {
		t.Fatalf("replay flag must be preserved, got: %s", events[0])
	}
}

func TestJournalReplayTwoRunsSameKeysDontCollide(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-alpha")
	_ = store.RecordRun("run-beta")
	_ = store.AppendEvent("run-alpha", 0, `{"type":"activity_recorded","key":"shared-key","run":"alpha"}`)
	_ = store.AppendEvent("run-beta", 0, `{"type":"activity_recorded","key":"shared-key","run":"beta"}`)

	alphaEvents, _ := store.EventsForRun("run-alpha")
	betaEvents, _ := store.EventsForRun("run-beta")

	if len(alphaEvents) != 1 || len(betaEvents) != 1 {
		t.Fatalf("each run must have exactly 1 event, got: alpha=%d beta=%d", len(alphaEvents), len(betaEvents))
	}
	if !strings.Contains(alphaEvents[0], `"run":"alpha"`) {
		t.Fatalf("alpha event must contain run identifier, got: %s", alphaEvents[0])
	}
	if !strings.Contains(betaEvents[0], `"run":"beta"`) {
		t.Fatalf("beta event must contain run identifier, got: %s", betaEvents[0])
	}
}

func TestJournalReplayRunAfterConformanceSuiteStored(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("conformance-replay")
	for i := 0; i < 3; i++ {
		_ = store.AppendEvent("conformance-replay", i, `{"type":"activity_recorded","key":"step"}`)
	}

	count, _ := store.EventCount("conformance-replay")
	if count != 3 {
		t.Fatalf("expected 3 journal entries, got: %d", count)
	}
}

func TestJournalReplayDeleteRunClearsJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("replay-del")
	for i := 0; i < 5; i++ {
		_ = store.AppendEvent("replay-del", i, `{"type":"activity_recorded"}`)
	}

	_ = store.DeleteRun("replay-del")

	has, _ := store.HasRun("replay-del")
	if has {
		t.Fatal("run must not exist after DeleteRun")
	}

	events, _ := store.EventsForRun("replay-del")
	if len(events) != 0 {
		t.Fatalf("events must be empty after DeleteRun, got: %d", len(events))
	}
}

func TestJournalReplayEventOrderIsStable(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("replay-order")
	keys := []string{"k-0", "k-1", "k-2", "k-3", "k-4"}
	for i, k := range keys {
		_ = store.AppendEvent("replay-order", i, `{"key":"`+k+`"}`)
	}

	events, _ := store.EventsForRun("replay-order")
	if len(events) != len(keys) {
		t.Fatalf("expected %d events, got: %d", len(keys), len(events))
	}
	for i, k := range keys {
		if !strings.Contains(events[i], k) {
			t.Fatalf("event %d must contain key %q, got: %s", i, k, events[i])
		}
	}
}
