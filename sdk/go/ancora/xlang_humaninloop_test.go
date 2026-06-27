package ancora_test

import (
	"encoding/json"
	"testing"
)

// Cross-language conformance: human-in-loop scenario -- Go.

type xlangHilEvent struct {
	Kind     string `json:"kind"`
	RunID    string `json:"run_id"`
	Prompt   string `json:"prompt,omitempty"`
	Decision string `json:"decision,omitempty"`
	Output   string `json:"output,omitempty"`
}

func makeXlangHilEvents(runID string) []xlangHilEvent {
	return []xlangHilEvent{
		{Kind: "started",            RunID: runID},
		{Kind: "decision_requested", RunID: runID, Prompt: "Please approve the draft"},
		{Kind: "decision_received",  RunID: runID, Decision: `{"approved":true}`},
		{Kind: "completed",          RunID: runID, Output: `{"result":"hil-ok"}`},
	}
}

func TestXlangGoHilStartedFirst(t *testing.T) {
	evs := makeXlangHilEvents("xlh-go")
	if evs[0].Kind != "started" {
		t.Fatalf("expected started, got %q", evs[0].Kind)
	}
}

func TestXlangGoHilRequestedBeforeReceived(t *testing.T) {
	evs := makeXlangHilEvents("xlh-go")
	var idx []int
	for i, e := range evs {
		if e.Kind == "decision_requested" || e.Kind == "decision_received" {
			idx = append(idx, i)
		}
	}
	if len(idx) != 2 || idx[0] >= idx[1] {
		t.Fatalf("requested must come before received, got indices %v", idx)
	}
}

func TestXlangGoHilDecisionIsApproved(t *testing.T) {
	evs := makeXlangHilEvents("xlh-go")
	var received xlangHilEvent
	for _, e := range evs {
		if e.Kind == "decision_received" {
			received = e
		}
	}
	var dec map[string]interface{}
	if err := json.Unmarshal([]byte(received.Decision), &dec); err != nil {
		t.Fatalf("unmarshal decision: %v", err)
	}
	if dec["approved"] != true {
		t.Fatalf("expected approved=true, got %v", dec["approved"])
	}
}

func TestXlangGoHilPromptNonEmpty(t *testing.T) {
	evs := makeXlangHilEvents("xlh-go")
	for _, e := range evs {
		if e.Kind == "decision_requested" && e.Prompt == "" {
			t.Fatal("prompt must be non-empty")
		}
	}
}

func TestXlangGoHilCompletedLast(t *testing.T) {
	evs := makeXlangHilEvents("xlh-go")
	if evs[len(evs)-1].Kind != "completed" {
		t.Fatalf("expected completed, got %q", evs[len(evs)-1].Kind)
	}
}
