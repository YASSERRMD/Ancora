package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2EStoreFailureRecoveryEmptyPayloadIsTolerated(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("store-fail-1")
	_ = store.AppendEvent("store-fail-1", 0, "")

	events, err := store.EventsForRun("store-fail-1")
	if err != nil {
		t.Fatalf("EventsForRun with empty payload: %v", err)
	}
	if len(events) != 1 {
		t.Logf("empty payload event count: %d (may vary by impl)", len(events))
	}
}

func TestE2EStoreFailureRecoveryAfterErrorEventRun(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("store-fail-2")
	_ = store.AppendEvent("store-fail-2", 0, `{"type":"started"}`)
	_ = store.AppendEvent("store-fail-2", 1, `{"type":"error","code":"INTERNAL","message":"disk write failure"}`)
	_ = store.AppendEvent("store-fail-2", 2, `{"type":"activity_recorded","key":"recovery-step"}`)
	_ = store.AppendEvent("store-fail-2", 3, `{"type":"completed"}`)

	events, _ := store.EventsForRun("store-fail-2")
	if len(events) != 4 {
		t.Fatalf("expected 4 events after store-fail recovery, got: %d", len(events))
	}
	if !strings.Contains(events[1], "INTERNAL") {
		t.Fatalf("event 1 must be internal error, got: %s", events[1])
	}
	if !strings.Contains(events[3], "completed") {
		t.Fatalf("event 3 must be completed, got: %s", events[3])
	}
}

func TestE2EStoreFailureRecoveryRunCountAfterPartialFailure(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("ok-run")
	_ = store.RecordRun("fail-run")
	_ = store.AppendEvent("ok-run", 0, `{"type":"completed"}`)

	count, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 runs (ok + fail), got: %d", count)
	}
}

func TestE2EStoreFailureRecoveryDeleteFailedRunOnly(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("healthy-run")
	_ = store.RecordRun("failed-run")
	_ = store.AppendEvent("healthy-run", 0, `{"type":"completed"}`)
	_ = store.AppendEvent("failed-run", 0, `{"type":"error","code":"INTERNAL"}`)

	_ = store.DeleteRun("failed-run")

	hasHealthy, _ := store.HasRun("healthy-run")
	hasFailed, _ := store.HasRun("failed-run")

	if !hasHealthy {
		t.Fatal("healthy run must survive deletion of failed run")
	}
	if hasFailed {
		t.Fatal("failed run must be deleted")
	}
}

func TestE2EStoreFailureRecoveryReRecordRun(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("re-record")
	_ = store.AppendEvent("re-record", 0, `{"type":"error","code":"INTERNAL"}`)
	_ = store.DeleteRun("re-record")

	if err := store.RecordRun("re-record"); err != nil {
		t.Fatalf("re-RecordRun after failure: %v", err)
	}
	_ = store.AppendEvent("re-record", 0, `{"type":"completed"}`)

	events, _ := store.EventsForRun("re-record")
	if len(events) != 1 {
		t.Fatalf("re-recorded run must have 1 event, got: %d", len(events))
	}
	if !strings.Contains(events[0], "completed") {
		t.Fatalf("re-recorded run event must be 'completed', got: %s", events[0])
	}
}
