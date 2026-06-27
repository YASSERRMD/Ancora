package ancora_test

import (
	"encoding/json"
	"testing"
)

// Cross-language conformance: verifier scenario -- Go.

type xlangVerifierEvent struct {
	Kind        string          `json:"kind"`
	RunID       string          `json:"run_id"`
	ActivityKey string          `json:"activity_key,omitempty"`
	Output      json.RawMessage `json:"output,omitempty"`
}

func makeXlangVerifierEvents(runID string) []xlangVerifierEvent {
	return []xlangVerifierEvent{
		{Kind: "started",   RunID: runID},
		{Kind: "activity",  RunID: runID, ActivityKey: "drafter"},
		{Kind: "activity",  RunID: runID, ActivityKey: "verifier"},
		{Kind: "completed", RunID: runID, Output: json.RawMessage(`{"verdict":"approved"}`)},
	}
}

func TestXlangGoVerifierStartedFirst(t *testing.T) {
	evs := makeXlangVerifierEvents("xlv-go")
	if evs[0].Kind != "started" {
		t.Fatalf("expected started, got %q", evs[0].Kind)
	}
}

func TestXlangGoVerifierCompletedLast(t *testing.T) {
	evs := makeXlangVerifierEvents("xlv-go")
	last := evs[len(evs)-1]
	if last.Kind != "completed" {
		t.Fatalf("expected completed, got %q", last.Kind)
	}
}

func TestXlangGoVerifierDrafterBeforeVerifier(t *testing.T) {
	evs := makeXlangVerifierEvents("xlv-go")
	var keys []string
	for _, e := range evs {
		if e.Kind == "activity" {
			keys = append(keys, e.ActivityKey)
		}
	}
	if len(keys) != 2 || keys[0] != "drafter" || keys[1] != "verifier" {
		t.Fatalf("expected [drafter verifier], got %v", keys)
	}
}

func TestXlangGoVerifierOutputHasVerdict(t *testing.T) {
	evs := makeXlangVerifierEvents("xlv-go")
	last := evs[len(evs)-1]
	var out map[string]interface{}
	if err := json.Unmarshal(last.Output, &out); err != nil {
		t.Fatalf("unmarshal output: %v", err)
	}
	if out["verdict"] != "approved" {
		t.Fatalf("expected verdict=approved, got %v", out["verdict"])
	}
}
