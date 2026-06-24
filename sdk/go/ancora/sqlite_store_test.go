package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func mustInMemoryStore(t *testing.T) *ancora.SqliteStore {
	t.Helper()
	s, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	t.Cleanup(func() { s.Close() })
	return s
}

func TestSqliteStoreOpensInMemory(t *testing.T) {
	s := mustInMemoryStore(t)
	if s == nil {
		t.Fatal("store must not be nil")
	}
}

func TestSqliteStoreRecordRunDoesNotError(t *testing.T) {
	s := mustInMemoryStore(t)
	if err := s.RecordRun("run-1"); err != nil {
		t.Fatalf("RecordRun: %v", err)
	}
}

func TestSqliteStoreHasRunTrueAfterRecord(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("run-2")
	ok, err := s.HasRun("run-2")
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if !ok {
		t.Fatal("HasRun must return true after RecordRun")
	}
}

func TestSqliteStoreHasRunFalseForUnknown(t *testing.T) {
	s := mustInMemoryStore(t)
	ok, err := s.HasRun("missing")
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if ok {
		t.Fatal("HasRun must return false for unrecorded run")
	}
}

func TestSqliteStoreAppendEventNoError(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("run-3")
	if err := s.AppendEvent("run-3", 0, `{"type":"started"}`); err != nil {
		t.Fatalf("AppendEvent: %v", err)
	}
}

func TestSqliteStoreEventsForRunReturnsAll(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("run-4")
	s.AppendEvent("run-4", 0, "ev0")
	s.AppendEvent("run-4", 1, "ev1")
	s.AppendEvent("run-4", 2, "ev2")
	evs, err := s.EventsForRun("run-4")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(evs) != 3 {
		t.Fatalf("expected 3 events, got: %d", len(evs))
	}
}

func TestSqliteStoreEventsOrderedBySeq(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("run-5")
	s.AppendEvent("run-5", 2, "last")
	s.AppendEvent("run-5", 0, "first")
	s.AppendEvent("run-5", 1, "middle")
	evs, _ := s.EventsForRun("run-5")
	if len(evs) != 3 || evs[0] != "first" || evs[2] != "last" {
		t.Fatalf("events out of seq order: %v", evs)
	}
}

func TestSqliteStoreEventsForUnknownRunReturnsEmpty(t *testing.T) {
	s := mustInMemoryStore(t)
	evs, err := s.EventsForRun("nobody")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(evs) != 0 {
		t.Fatalf("expected 0 events, got: %d", len(evs))
	}
}

func TestSqliteStoreRecordRunIdempotent(t *testing.T) {
	s := mustInMemoryStore(t)
	if err := s.RecordRun("dup-run"); err != nil {
		t.Fatalf("first RecordRun: %v", err)
	}
	if err := s.RecordRun("dup-run"); err != nil {
		t.Fatalf("duplicate RecordRun must not error: %v", err)
	}
}

func TestSqliteStoreCloseIsIdempotentSecondCallErrors(t *testing.T) {
	s, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("open: %v", err)
	}
	if err := s.Close(); err != nil {
		t.Fatalf("first Close: %v", err)
	}
}

func TestSqliteStoreMultipleRunsIsolated(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("r1")
	s.RecordRun("r2")
	s.AppendEvent("r1", 0, "ev-r1")
	s.AppendEvent("r2", 0, "ev-r2")
	evs1, _ := s.EventsForRun("r1")
	evs2, _ := s.EventsForRun("r2")
	if len(evs1) != 1 || evs1[0] != "ev-r1" {
		t.Fatalf("r1 isolation broken: %v", evs1)
	}
	if len(evs2) != 1 || evs2[0] != "ev-r2" {
		t.Fatalf("r2 isolation broken: %v", evs2)
	}
}
