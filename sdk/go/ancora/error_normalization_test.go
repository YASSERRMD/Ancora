package ancora_test

import (
	"errors"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestErrorInternalIsNonZero(t *testing.T) {
	if ancora.ErrInternal == ancora.ErrOk {
		t.Fatal("ErrInternal must be distinct from ErrOk")
	}
}

func TestErrorInternalHasMessage(t *testing.T) {
	if ancora.ErrInternal.Error() == "" {
		t.Fatal("ErrInternal must have a non-empty message")
	}
}

func TestErrorUnregisteredToolReturnsErrInternal(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	_, err := reg.Invoke("missing-tool", []byte(`{}`))
	if err == nil {
		t.Fatal("invoking unregistered tool must return error")
	}
	if !errors.Is(err, ancora.ErrInternal) {
		t.Fatalf("expected ErrInternal, got: %v", err)
	}
}

func TestErrorToolCallbackCanReturnCustomError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	sentinel := errors.New("tool-specific-error")
	reg.Register("error-tool", func(input []byte) ([]byte, error) {
		return nil, sentinel
	})

	_, err := reg.Invoke("error-tool", []byte(`{}`))
	if err == nil {
		t.Fatal("error tool must return an error")
	}
	if !errors.Is(err, sentinel) {
		t.Fatalf("expected sentinel error, got: %v", err)
	}
}

func TestErrorToolCallbackNilOutputWithNilError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("nil-output", func(input []byte) ([]byte, error) {
		return nil, nil
	})
	out, err := reg.Invoke("nil-output", []byte(`{}`))
	if err != nil {
		t.Fatalf("nil-output tool must not return error: %v", err)
	}
	if out != nil {
		t.Logf("nil-output tool returned: %s (both nil and empty are acceptable)", out)
	}
}

func TestErrorNormalizationStoreRecordsErrorEvent(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("error-run")
	ev := `{"type":"error","code":"TOOL_ERROR","message":"tool returned error","detail":""}`
	_ = store.AppendEvent("error-run", 0, ev)

	events, _ := store.EventsForRun("error-run")
	if len(events) != 1 {
		t.Fatalf("expected 1 error event, got: %d", len(events))
	}
	if !strings.Contains(events[0], "TOOL_ERROR") {
		t.Fatalf("event must contain 'TOOL_ERROR', got: %s", events[0])
	}
}

func TestErrorNormalizationErrorEventHasCode(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("error-code-run")
	_ = store.AppendEvent("error-code-run", 0, `{"type":"error","code":"RATE_LIMIT","message":"429","detail":"retry after 1s"}`)

	events, _ := store.EventsForRun("error-code-run")
	if !strings.Contains(events[0], "RATE_LIMIT") {
		t.Fatalf("error event must contain 'RATE_LIMIT', got: %s", events[0])
	}
}

func TestErrorNormalizationErrorEventHasMessage(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("error-msg-run")
	_ = store.AppendEvent("error-msg-run", 0, `{"type":"error","code":"CONTEXT_LIMIT","message":"context window exceeded","detail":""}`)

	events, _ := store.EventsForRun("error-msg-run")
	if !strings.Contains(events[0], "context window exceeded") {
		t.Fatalf("error event must contain message, got: %s", events[0])
	}
}

func TestErrorNormalizationMultipleErrorEvents(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("multi-error")
	codes := []string{"RATE_LIMIT", "TOOL_ERROR", "CONTEXT_LIMIT"}
	for i, code := range codes {
		_ = store.AppendEvent("multi-error", i, `{"type":"error","code":"`+code+`","message":"","detail":""}`)
	}

	events, _ := store.EventsForRun("multi-error")
	if len(events) != 3 {
		t.Fatalf("expected 3 error events, got: %d", len(events))
	}
}

func TestErrorNormalizationStartRunWithBadSpecDoesNotPanic(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, err := rt.StartRun([]byte(`invalid-json`))
	if err != nil {
		t.Logf("StartRun with invalid JSON returned error (acceptable): %v", err)
		return
	}
	if run.ID() != "" {
		t.Logf("StartRun with invalid JSON returned run ID: %s", run.ID())
	}
}
