package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2ERateLimitEventIsRecorded(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rl-run-1")
	_ = store.AppendEvent("rl-run-1", 0, `{"type":"error","code":"RATE_LIMIT","retry_after_ms":500}`)

	events, _ := store.EventsForRun("rl-run-1")
	if len(events) != 1 {
		t.Fatalf("expected 1 rate-limit event, got: %d", len(events))
	}
	if !strings.Contains(events[0], "RATE_LIMIT") {
		t.Fatalf("event must contain RATE_LIMIT, got: %s", events[0])
	}
}

func TestE2ERateLimitRetryAfterMSIsPresent(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rl-run-2")
	_ = store.AppendEvent("rl-run-2", 0, `{"type":"error","code":"RATE_LIMIT","retry_after_ms":1000}`)

	events, _ := store.EventsForRun("rl-run-2")
	if !strings.Contains(events[0], "retry_after_ms") {
		t.Fatalf("rate-limit event must contain 'retry_after_ms', got: %s", events[0])
	}
}

func TestE2ERateLimitBurstFiveConsecutiveEvents(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rl-burst")
	for i := 0; i < 5; i++ {
		_ = store.AppendEvent("rl-burst", i, `{"type":"error","code":"RATE_LIMIT","retry_after_ms":100}`)
	}

	count, _ := store.EventCount("rl-burst")
	if count != 5 {
		t.Fatalf("expected 5 rate-limit burst events, got: %d", count)
	}
}

func TestE2ERateLimitFollowedBySuccess(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rl-success")
	_ = store.AppendEvent("rl-success", 0, `{"type":"error","code":"RATE_LIMIT","retry_after_ms":100}`)
	_ = store.AppendEvent("rl-success", 1, `{"type":"error","code":"RATE_LIMIT","retry_after_ms":200}`)
	_ = store.AppendEvent("rl-success", 2, `{"type":"activity_recorded","key":"step-1"}`)
	_ = store.AppendEvent("rl-success", 3, `{"type":"completed"}`)

	events, _ := store.EventsForRun("rl-success")
	if len(events) != 4 {
		t.Fatalf("expected 4 events (2 retries + activity + completed), got: %d", len(events))
	}
	if !strings.Contains(events[3], "completed") {
		t.Fatalf("final event must be 'completed', got: %s", events[3])
	}
}

func TestE2ERateLimitToolRegistryHandlesRetry(t *testing.T) {
	calls := 0
	reg := ancora.NewGoToolRegistry()
	reg.Register("rate-limited-tool", func(input []byte) ([]byte, error) {
		calls++
		if calls < 3 {
			return []byte(`{"error":"rate_limited","retry_after_ms":100}`), nil
		}
		return []byte(`{"result":"success"}`), nil
	})

	for i := 0; i < 3; i++ {
		out, err := reg.Invoke("rate-limited-tool", []byte(`{}`))
		if err != nil {
			t.Fatalf("Invoke %d: %v", i, err)
		}
		_ = out
	}
	if calls != 3 {
		t.Fatalf("expected 3 calls (2 rate-limited + 1 success), got: %d", calls)
	}
}

func TestE2ERateLimitBurstAndRecoveryEventCount(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rl-recovery")
	for i := 0; i < 3; i++ {
		_ = store.AppendEvent("rl-recovery", i, `{"type":"error","code":"RATE_LIMIT"}`)
	}
	_ = store.AppendEvent("rl-recovery", 3, `{"type":"activity_recorded","key":"k1"}`)
	_ = store.AppendEvent("rl-recovery", 4, `{"type":"completed"}`)

	count, _ := store.EventCount("rl-recovery")
	if count != 5 {
		t.Fatalf("expected 5 events (3 RL + activity + completed), got: %d", count)
	}
}

func TestE2ERateLimitTwoRunsWithIndependentBursts(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rl-A")
	_ = store.RecordRun("rl-B")
	for i := 0; i < 3; i++ {
		_ = store.AppendEvent("rl-A", i, `{"type":"error","code":"RATE_LIMIT"}`)
	}
	for i := 0; i < 2; i++ {
		_ = store.AppendEvent("rl-B", i, `{"type":"error","code":"RATE_LIMIT"}`)
	}

	countA, _ := store.EventCount("rl-A")
	countB, _ := store.EventCount("rl-B")
	if countA != 3 {
		t.Fatalf("rl-A expected 3 events, got: %d", countA)
	}
	if countB != 2 {
		t.Fatalf("rl-B expected 2 events, got: %d", countB)
	}
}
