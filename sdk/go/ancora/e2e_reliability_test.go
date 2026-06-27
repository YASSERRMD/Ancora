package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// TestE2EReliabilityZeroDuplicateSideEffects verifies that the same run ID
// cannot be recorded twice without overwriting or producing duplicates.
func TestE2EReliabilityZeroDuplicateSideEffects(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("idem-run")
	_ = store.RecordRun("idem-run")

	count, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if count > 1 {
		t.Fatalf("recording same run twice must not create duplicates, got: %d", count)
	}
}

func TestE2EReliabilityStoreFailureRecoveryWithInMemory(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("fail-recovery")
	_ = store.AppendEvent("fail-recovery", 0, `{"type":"started"}`)
	_ = store.AppendEvent("fail-recovery", 1, `{"type":"error","code":"INTERNAL"}`)
	_ = store.AppendEvent("fail-recovery", 2, `{"type":"completed"}`)

	events, _ := store.EventsForRun("fail-recovery")
	if len(events) != 3 {
		t.Fatalf("expected 3 events including error, got: %d", len(events))
	}
	if !strings.Contains(events[1], "INTERNAL") {
		t.Fatalf("error event must be recorded, got: %s", events[1])
	}
}

func TestE2EReliabilityRateLimitHandlingStoreEvents(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rate-limit-run")
	retryPayloads := []string{
		`{"type":"error","code":"RATE_LIMIT","retry_after_ms":1000}`,
		`{"type":"error","code":"RATE_LIMIT","retry_after_ms":2000}`,
		`{"type":"activity_recorded","key":"step-1","replayed":false}`,
		`{"type":"completed"}`,
	}
	for i, p := range retryPayloads {
		_ = store.AppendEvent("rate-limit-run", i, p)
	}

	events, _ := store.EventsForRun("rate-limit-run")
	if len(events) != 4 {
		t.Fatalf("expected 4 events with rate-limit retries, got: %d", len(events))
	}

	rateLimitCount := 0
	for _, ev := range events {
		if strings.Contains(ev, "RATE_LIMIT") {
			rateLimitCount++
		}
	}
	if rateLimitCount != 2 {
		t.Fatalf("expected 2 RATE_LIMIT events, got: %d", rateLimitCount)
	}
}

func TestE2EReliabilityLongRunStabilityTenRuns(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	const numRuns = 10
	for i := 0; i < numRuns; i++ {
		id := "long-run-" + string(rune('A'+i))
		_ = store.RecordRun(id)
		for j := 0; j < 20; j++ {
			_ = store.AppendEvent(id, j, `{"type":"activity_recorded","key":"step"}`)
		}
	}

	count, _ := store.RunCount()
	if count != numRuns {
		t.Fatalf("expected %d stable runs, got: %d", numRuns, count)
	}

	for i := 0; i < numRuns; i++ {
		id := "long-run-" + string(rune('A'+i))
		c, _ := store.EventCount(id)
		if c != 20 {
			t.Fatalf("run %s expected 20 events, got: %d", id, c)
		}
	}
}

func TestE2EReliabilityToolRegistryIsStableAfterManyRegistrations(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	const numTools = 100
	for i := 0; i < numTools; i++ {
		name := "tool-" + string(rune('A'+i%26)) + string(rune('a'+i/26))
		reg.Register(name, func(input []byte) ([]byte, error) {
			return []byte(`{}`), nil
		})
	}

	if reg.Count() != numTools {
		t.Fatalf("expected %d tools, got: %d", numTools, reg.Count())
	}
}

func TestE2EReliabilityRuntimeFreedThenNewOneStarts(t *testing.T) {
	rt1 := mustRuntime(t)
	rt1.Free()

	rt2 := mustRuntime(t)
	defer rt2.Free()

	run, err := rt2.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun after Free+New: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty on new Runtime after previous was Freed")
	}
}

func TestE2EReliabilityStoreDeleteAndReRecordRun(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("recycle-run")
	_ = store.AppendEvent("recycle-run", 0, `{"type":"started"}`)
	_ = store.DeleteRun("recycle-run")

	if err := store.RecordRun("recycle-run"); err != nil {
		t.Fatalf("re-RecordRun after Delete: %v", err)
	}
	_ = store.AppendEvent("recycle-run", 0, `{"type":"started"}`)

	count, _ := store.EventCount("recycle-run")
	if count != 1 {
		t.Fatalf("re-recorded run must have 1 event, got: %d", count)
	}
}
