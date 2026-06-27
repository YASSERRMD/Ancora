package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

// TestE2ERestartRecoveryJournalSurvivesNewStore simulates a "restart"
// by writing events to one store, closing it, reopening it, and verifying
// the events are still present. Uses a temp file path via t.TempDir().
func TestE2ERestartRecoveryJournalSurvivesNewStore(t *testing.T) {
	path := t.TempDir() + "/restart.db"

	store1, err := ancora.OpenSqliteStore(path)
	if err != nil {
		t.Fatalf("OpenSqliteStore (first): %v", err)
	}

	_ = store1.RecordRun("recovery-run")
	_ = store1.AppendEvent("recovery-run", 0, `{"type":"started"}`)
	_ = store1.AppendEvent("recovery-run", 1, `{"type":"activity_recorded","key":"step-1"}`)
	store1.Close()

	store2, err := ancora.OpenSqliteStore(path)
	if err != nil {
		t.Fatalf("OpenSqliteStore (second): %v", err)
	}
	defer store2.Close()

	has, err := store2.HasRun("recovery-run")
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if !has {
		t.Fatal("run must persist across store restart")
	}

	events, err := store2.EventsForRun("recovery-run")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) != 2 {
		t.Fatalf("expected 2 persisted events after restart, got: %d", len(events))
	}
}

func TestE2ERestartRecoveryEventCountPersists(t *testing.T) {
	path := t.TempDir() + "/event-count.db"

	store1, _ := ancora.OpenSqliteStore(path)
	_ = store1.RecordRun("count-run")
	for i := 0; i < 5; i++ {
		_ = store1.AppendEvent("count-run", i, `{"n":1}`)
	}
	store1.Close()

	store2, _ := ancora.OpenSqliteStore(path)
	defer store2.Close()

	count, err := store2.EventCount("count-run")
	if err != nil {
		t.Fatalf("EventCount after restart: %v", err)
	}
	if count != 5 {
		t.Fatalf("expected 5 events after restart, got: %d", count)
	}
}

func TestE2ERestartRecoveryMultipleRunsPersist(t *testing.T) {
	path := t.TempDir() + "/multi-run.db"

	store1, _ := ancora.OpenSqliteStore(path)
	for _, id := range []string{"run-x", "run-y", "run-z"} {
		_ = store1.RecordRun(id)
	}
	store1.Close()

	store2, _ := ancora.OpenSqliteStore(path)
	defer store2.Close()

	count, err := store2.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count != 3 {
		t.Fatalf("expected 3 runs after restart, got: %d", count)
	}
}

func TestE2ERestartRecoveryListRunsAfterRestart(t *testing.T) {
	path := t.TempDir() + "/list-runs.db"

	store1, _ := ancora.OpenSqliteStore(path)
	_ = store1.RecordRun("persist-a")
	_ = store1.RecordRun("persist-b")
	store1.Close()

	store2, _ := ancora.OpenSqliteStore(path)
	defer store2.Close()

	ids, err := store2.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	found := make(map[string]bool)
	for _, id := range ids {
		found[id] = true
	}
	if !found["persist-a"] {
		t.Fatal("persist-a must survive restart")
	}
	if !found["persist-b"] {
		t.Fatal("persist-b must survive restart")
	}
}

func TestE2ERestartRecoveryStoringTransportWritesPersist(t *testing.T) {
	path := t.TempDir() + "/storing.db"

	store1, _ := ancora.OpenSqliteStore(path)
	rt := mustRuntime(t)
	defer rt.Free()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store1)
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
	store1.Close()

	store2, _ := ancora.OpenSqliteStore(path)
	defer store2.Close()

	has, _ := store2.HasRun(runID)
	if !has {
		t.Fatalf("run %q must persist after StoringTransport restart", runID)
	}
}

func TestE2ERestartRecoveryDeleteRunClearsAcrossInstances(t *testing.T) {
	path := t.TempDir() + "/delete.db"

	store1, _ := ancora.OpenSqliteStore(path)
	_ = store1.RecordRun("del-run")
	_ = store1.AppendEvent("del-run", 0, `{"type":"started"}`)
	store1.Close()

	store2, _ := ancora.OpenSqliteStore(path)
	defer store2.Close()

	_ = store2.DeleteRun("del-run")
	has, _ := store2.HasRun("del-run")
	if has {
		t.Fatal("deleted run must not exist after delete")
	}
}
