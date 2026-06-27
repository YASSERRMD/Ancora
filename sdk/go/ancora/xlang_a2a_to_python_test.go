package ancora_test

import (
	"encoding/json"
	"testing"
)

// Cross-language A2A: Go hands off to Python over A2A (offline fixture).

type a2aEnvelope struct {
	Protocol  string                 `json:"protocol"`
	Sender    map[string]string      `json:"sender"`
	Recipient map[string]string      `json:"recipient"`
	RunID     string                 `json:"run_id"`
	Payload   map[string]interface{} `json:"payload"`
}

const a2aGoToPythonRunID = "a2a-go-python-001"

func makeGoToPythonEnvelope() a2aEnvelope {
	return a2aEnvelope{
		Protocol:  "a2a/1.0",
		Sender:    map[string]string{"lang": "go", "sdk_version": "0.3.0"},
		Recipient: map[string]string{"lang": "python", "sdk_version": "0.3.0"},
		RunID:     a2aGoToPythonRunID,
		Payload: map[string]interface{}{
			"task":           "analyse",
			"data":           "Go produced this data",
			"handoff_reason": "analyser runs in Python",
		},
	}
}

func makeGoToPythonResponse() a2aEnvelope {
	return a2aEnvelope{
		Protocol:  "a2a/1.0",
		Sender:    map[string]string{"lang": "python"},
		Recipient: map[string]string{"lang": "go"},
		RunID:     a2aGoToPythonRunID,
		Payload:   map[string]interface{}{"status": "analysed", "result": "python-ok"},
	}
}

func TestA2AGoToPythonProtocol(t *testing.T) {
	env := makeGoToPythonEnvelope()
	if env.Protocol != "a2a/1.0" {
		t.Fatalf("expected a2a/1.0, got %q", env.Protocol)
	}
}

func TestA2AGoToPythonSenderIsGo(t *testing.T) {
	env := makeGoToPythonEnvelope()
	if env.Sender["lang"] != "go" {
		t.Fatalf("expected sender=go, got %q", env.Sender["lang"])
	}
}

func TestA2AGoToPythonRecipientIsPython(t *testing.T) {
	env := makeGoToPythonEnvelope()
	if env.Recipient["lang"] != "python" {
		t.Fatalf("expected recipient=python, got %q", env.Recipient["lang"])
	}
}

func TestA2AGoToPythonResponseRunIDMatches(t *testing.T) {
	env := makeGoToPythonEnvelope()
	res := makeGoToPythonResponse()
	if res.RunID != env.RunID {
		t.Fatalf("run_id mismatch: %q vs %q", res.RunID, env.RunID)
	}
}

func TestA2AGoToPythonResponseSenderIsPython(t *testing.T) {
	res := makeGoToPythonResponse()
	if res.Sender["lang"] != "python" {
		t.Fatalf("expected response sender=python, got %q", res.Sender["lang"])
	}
}

func TestA2AGoToPythonEnvelopeSerialisesToJSON(t *testing.T) {
	env := makeGoToPythonEnvelope()
	raw, err := json.Marshal(env)
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	var decoded a2aEnvelope
	if err := json.Unmarshal(raw, &decoded); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if decoded.Protocol != "a2a/1.0" {
		t.Fatalf("roundtrip protocol mismatch: %q", decoded.Protocol)
	}
}

func TestA2AGoToPythonPayloadHasHandoffReason(t *testing.T) {
	env := makeGoToPythonEnvelope()
	if _, ok := env.Payload["handoff_reason"]; !ok {
		t.Fatal("payload must have handoff_reason")
	}
}
