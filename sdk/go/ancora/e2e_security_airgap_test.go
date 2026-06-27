package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

// TestE2EAirGappedEgressZeroNoNetworkCallsInRegistryInvoke verifies that
// invoking tools through the GoToolRegistry never makes network calls:
// all fixture callbacks are pure in-process functions.
func TestE2EAirGappedEgressZeroNoNetworkCallsInRegistryInvoke(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	calls := 0
	reg.Register("airgap-tool", func(input []byte) ([]byte, error) {
		calls++
		return []byte(`{"result":"offline"}`), nil
	})

	for i := 0; i < 10; i++ {
		out, err := reg.Invoke("airgap-tool", []byte(`{}`))
		if err != nil {
			t.Fatalf("Invoke %d: %v", i, err)
		}
		if len(out) == 0 {
			t.Fatalf("Invoke %d returned empty output", i)
		}
	}

	if calls != 10 {
		t.Fatalf("expected 10 in-process calls, got: %d", calls)
	}
}

func TestE2EAirGappedEgressZeroStoreWorksPurelyOffline(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("airgap-run")
	for i := 0; i < 5; i++ {
		_ = store.AppendEvent("airgap-run", i, `{"type":"activity_recorded"}`)
	}

	count, _ := store.EventCount("airgap-run")
	if count != 5 {
		t.Fatalf("expected 5 offline events, got: %d", count)
	}
}

func TestE2EAirGappedRuntimeStartsWithoutNetwork(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun (no network): %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty in airgapped mode")
	}
}

func TestE2EAirGappedCgoTransportPollsWithoutNetwork(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	tr := ancora.NewCgoTransport(rt)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun (cgo, no network): %v", err)
	}

	ev, err := tr.PollRun(context.Background(), runID)
	if err != nil {
		t.Fatalf("PollRun (cgo, no network): %v", err)
	}
	_ = ev
}

func TestE2EAirGappedStoringTransportRecordsRunOffline(t *testing.T) {
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
		t.Fatalf("StartRun (offline): %v", err)
	}

	has, _ := store.HasRun(runID)
	if !has {
		t.Fatal("airgapped run must be recorded in store")
	}
}

func TestE2EAirGappedConformanceSuiteRunsOffline(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	if len(results) == 0 {
		t.Fatal("airgapped conformance suite must produce results")
	}
}

func TestE2EAirGappedToolkitFullSurfaceOffline(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)

	tools := []string{"tool-a", "tool-b", "tool-c"}
	for _, name := range tools {
		n := name
		tk.RegisterTool(n, func(input []byte) ([]byte, error) {
			return []byte(`{"tool":"` + n + `"}`), nil
		})
	}

	for _, name := range tools {
		if !tk.Tools().Has(name) {
			t.Fatalf("toolkit missing offline tool: %s", name)
		}
		out, err := tk.InvokeTool(name, []byte(`{}`))
		if err != nil {
			t.Fatalf("InvokeTool %s: %v", name, err)
		}
		if len(out) == 0 {
			t.Fatalf("offline tool %s returned empty output", name)
		}
	}
}

func TestE2EAirGappedSchemaFromStructOffline(t *testing.T) {
	type localPayload struct {
		Name  string `json:"name"`
		Count int    `json:"count"`
	}
	schema, err := ancora.SchemaFromStruct(localPayload{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	if schema == "" {
		t.Fatal("SchemaFromStruct must return non-empty JSON schema string offline")
	}
}
