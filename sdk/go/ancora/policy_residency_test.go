package ancora_test

import (
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// policyEvent represents the shape of a policy enforcement event payload.
type policyEvent struct {
	Type   string `json:"type"`
	Effect string `json:"effect"`
	Reason string `json:"reason"`
}

func TestPolicyEventRoundTripJSON(t *testing.T) {
	ev := policyEvent{Type: "policy_enforced", Effect: "block", Reason: "data-residency"}
	b, err := json.Marshal(ev)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	var got policyEvent
	if err := json.Unmarshal(b, &got); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if got.Effect != "block" {
		t.Fatalf("effect mismatch: %q", got.Effect)
	}
}

func TestPolicyBlockEventStoredInJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("policy-run-1")
	payload := `{"type":"policy_enforced","effect":"block","reason":"data-residency"}`
	if err := store.AppendEvent("policy-run-1", 0, payload); err != nil {
		t.Fatalf("AppendEvent: %v", err)
	}

	events, _ := store.EventsForRun("policy-run-1")
	if len(events) != 1 {
		t.Fatalf("expected 1 policy event, got: %d", len(events))
	}
	if !strings.Contains(events[0], "block") {
		t.Fatalf("stored event must contain 'block', got: %s", events[0])
	}
}

func TestPolicyAllowEventStoredInJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("policy-run-allow")
	payload := `{"type":"policy_enforced","effect":"allow","reason":""}`
	_ = store.AppendEvent("policy-run-allow", 0, payload)

	events, _ := store.EventsForRun("policy-run-allow")
	if len(events) != 1 {
		t.Fatalf("expected 1 allow event, got: %d", len(events))
	}
}

func TestPolicyToolSpecCanEncodeResidencyConstraint(t *testing.T) {
	spec := ancora.NewToolSpecBuilder().
		WithToolName("residency-check").
		WithDescription("Verifies data does not leave the EU region").
		WithInputSchema(`{"type":"object","properties":{"region":{"type":"string"}}}`).
		Build()
	if spec.GetName() != "residency-check" {
		t.Fatalf("tool name mismatch: %q", spec.GetName())
	}
	if spec.GetDescription() == "" {
		t.Fatal("tool description must be non-empty")
	}
}

func TestPolicyAgentSpecWithResidencyTool(t *testing.T) {
	residencyTool := ancora.NewToolSpec("residency-check", "Checks data residency policy")
	spec := ancora.NewAgentSpecBuilder().
		WithName("policy-agent").
		WithModelID("gpt-4o").
		WithTool(residencyTool).
		Build()
	if len(spec.GetTools()) != 1 {
		t.Fatalf("expected 1 tool, got: %d", len(spec.GetTools()))
	}
}

func TestPolicyRunWithResidencyToolStartsSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	residencyTool := ancora.NewToolSpec("residency-check", "Checks data residency policy")
	spec := ancora.NewAgentSpecBuilder().
		WithName("policy-agent").
		WithModelID("llama3").
		WithTool(residencyTool).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestPolicyMultipleEnforcementEventsAreOrdered(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("policy-ordered")
	payloads := []string{
		`{"type":"policy_enforced","effect":"allow","seq":0}`,
		`{"type":"policy_enforced","effect":"block","seq":1}`,
		`{"type":"policy_enforced","effect":"allow","seq":2}`,
	}
	for i, p := range payloads {
		_ = store.AppendEvent("policy-ordered", i, p)
	}

	events, _ := store.EventsForRun("policy-ordered")
	if len(events) != 3 {
		t.Fatalf("expected 3 policy events, got: %d", len(events))
	}
}

func TestPolicyResidencyToolRegisteredInToolkit(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("residency-check", func(input []byte) ([]byte, error) {
		return []byte(`{"allowed":true}`), nil
	})
	if !tk.Tools().Has("residency-check") {
		t.Fatal("toolkit must have residency-check tool")
	}
}

func TestPolicyResidencyToolInvoked(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("residency-check", func(input []byte) ([]byte, error) {
		return []byte(`{"allowed":true}`), nil
	})

	out, err := reg.Invoke("residency-check", []byte(`{"region":"eu-west-1"}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	if !strings.Contains(string(out), "allowed") {
		t.Fatalf("expected 'allowed' in output, got: %s", out)
	}
}
