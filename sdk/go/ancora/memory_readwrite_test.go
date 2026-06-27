package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func TestMemoryStoreOpensInMemory(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()
}

func TestMemoryStoreRecordRunSucceeds(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	if err := store.RecordRun("run-001"); err != nil {
		t.Fatalf("RecordRun: %v", err)
	}
}

func TestMemoryStoreHasRunTrueAfterRecord(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-has-1")
	has, err := store.HasRun("run-has-1")
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if !has {
		t.Fatal("HasRun must return true after RecordRun")
	}
}

func TestMemoryStoreHasRunFalseForUnrecorded(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	has, err := store.HasRun("never-recorded")
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if has {
		t.Fatal("HasRun must return false for unrecorded run")
	}
}

func TestMemoryStoreAppendEventAndRead(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-ev")
	if err := store.AppendEvent("run-ev", 0, `{"type":"started"}`); err != nil {
		t.Fatalf("AppendEvent: %v", err)
	}

	events, err := store.EventsForRun("run-ev")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) != 1 {
		t.Fatalf("expected 1 event, got: %d", len(events))
	}
}

func TestMemoryStoreMultipleEventsPreservesOrder(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-order")
	payloads := []struct {
		seq     int
		payload string
	}{
		{0, `{"type":"started","seq":0}`},
		{1, `{"type":"token","seq":1}`},
		{2, `{"type":"completed","seq":2}`},
	}
	for _, p := range payloads {
		if err := store.AppendEvent("run-order", p.seq, p.payload); err != nil {
			t.Fatalf("AppendEvent seq %d: %v", p.seq, err)
		}
	}

	events, err := store.EventsForRun("run-order")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) != 3 {
		t.Fatalf("expected 3 events, got: %d", len(events))
	}
}

func TestMemoryStoreTwoRunsAreIsolated(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-A")
	_ = store.RecordRun("run-B")
	_ = store.AppendEvent("run-A", 0, `{"run":"A"}`)
	_ = store.AppendEvent("run-B", 0, `{"run":"B"}`)
	_ = store.AppendEvent("run-B", 1, `{"run":"B2"}`)

	evA, _ := store.EventsForRun("run-A")
	evB, _ := store.EventsForRun("run-B")

	if len(evA) != 1 {
		t.Fatalf("run-A expected 1 event, got: %d", len(evA))
	}
	if len(evB) != 2 {
		t.Fatalf("run-B expected 2 events, got: %d", len(evB))
	}
}

func TestMemoryStoreEventCountGrowsWithAppend(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-grow")
	for i := 0; i < 10; i++ {
		_ = store.AppendEvent("run-grow", i, `{"n":1}`)
	}

	count, err := store.EventCount("run-grow")
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if count != 10 {
		t.Fatalf("expected 10 events, got: %d", count)
	}
}

func TestMemoryStoreListRunsReturnedAfterRecord(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	for _, id := range []string{"r1", "r2", "r3"} {
		_ = store.RecordRun(id)
	}

	ids, err := store.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	if len(ids) < 3 {
		t.Fatalf("expected at least 3 run IDs, got: %d", len(ids))
	}

	found := make(map[string]bool)
	for _, id := range ids {
		found[id] = true
	}
	for _, expected := range []string{"r1", "r2", "r3"} {
		if !found[expected] {
			t.Fatalf("ListRuns missing expected run ID: %s", expected)
		}
	}
}

func TestMemoryStoreRunCountAfterRecord(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	for i := 0; i < 5; i++ {
		_ = store.RecordRun("run-count-" + string(rune('A'+i)))
	}
	count, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count != 5 {
		t.Fatalf("expected 5 runs, got: %d", count)
	}
}

func TestMemoryStoreDeleteRunRemovesIt(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("run-del")
	_ = store.DeleteRun("run-del")

	has, _ := store.HasRun("run-del")
	if has {
		t.Fatal("HasRun must return false after DeleteRun")
	}
}
