package ancora_test

import (
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// RAG retrieval tests validate the journal-level representation of a
// retrieval-augmented generation flow in the Go SDK. No live vector
// store is contacted; results come from tool fixture callbacks.

type ragChunk struct {
	Text  string  `json:"text"`
	Score float64 `json:"score"`
}

func fixtureRAGTool(input []byte) ([]byte, error) {
	chunks := []ragChunk{
		{Text: "Go uses goroutines for concurrency.", Score: 0.93},
		{Text: "Channels communicate between goroutines.", Score: 0.88},
		{Text: "The select statement chooses among channel operations.", Score: 0.82},
	}
	b, err := json.Marshal(chunks)
	if err != nil {
		return nil, err
	}
	return b, nil
}

func TestRAGToolReturnsJSONArray(t *testing.T) {
	out, err := fixtureRAGTool([]byte(`{"query":"goroutines","top_k":3}`))
	if err != nil {
		t.Fatalf("fixtureRAGTool: %v", err)
	}
	var chunks []ragChunk
	if err := json.Unmarshal(out, &chunks); err != nil {
		t.Fatalf("unmarshal RAG result: %v", err)
	}
	if len(chunks) != 3 {
		t.Fatalf("expected 3 chunks, got: %d", len(chunks))
	}
}

func TestRAGToolChunksHaveScores(t *testing.T) {
	out, _ := fixtureRAGTool(nil)
	var chunks []ragChunk
	_ = json.Unmarshal(out, &chunks)
	for i, chunk := range chunks {
		if chunk.Score <= 0 {
			t.Fatalf("chunk %d has non-positive score: %f", i, chunk.Score)
		}
	}
}

func TestRAGToolChunkScoresDescend(t *testing.T) {
	out, _ := fixtureRAGTool(nil)
	var chunks []ragChunk
	_ = json.Unmarshal(out, &chunks)
	for i := 1; i < len(chunks); i++ {
		if chunks[i].Score > chunks[i-1].Score {
			t.Fatalf("chunk scores must descend: [%d]=%f > [%d]=%f",
				i, chunks[i].Score, i-1, chunks[i-1].Score)
		}
	}
}

func TestRAGToolRegisteredInToolkit(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("vector-retrieve", fixtureRAGTool)
	if !tk.Tools().Has("vector-retrieve") {
		t.Fatal("toolkit must have vector-retrieve tool after registration")
	}
}

func TestRAGToolInvokedViaRegistry(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("vector-retrieve", fixtureRAGTool)

	out, err := reg.Invoke("vector-retrieve", []byte(`{"query":"goroutines","top_k":3}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	if len(out) == 0 {
		t.Fatal("RAG tool must return non-empty output")
	}
}

func TestRAGToolOutputContainsFirstChunkText(t *testing.T) {
	out, _ := fixtureRAGTool([]byte(`{}`))
	if !strings.Contains(string(out), "goroutines") {
		t.Fatalf("RAG output must contain 'goroutines', got: %s", out)
	}
}

func TestRAGAgentSpecIncludesRetrieveTool(t *testing.T) {
	retrieveTool := ancora.NewToolSpec("vector-retrieve", "retrieves relevant chunks from the vector store")
	spec := ancora.NewAgentSpecBuilder().
		WithName("rag-agent").
		WithModelID("gpt-4o").
		WithInstructions("You are a retrieval-augmented assistant.").
		WithTool(retrieveTool).
		Build()
	if len(spec.GetTools()) != 1 {
		t.Fatalf("expected 1 tool, got: %d", len(spec.GetTools()))
	}
	if spec.GetTools()[0].GetName() != "vector-retrieve" {
		t.Fatalf("tool name mismatch: %q", spec.GetTools()[0].GetName())
	}
}

func TestRAGRunWithRetrieveToolCanStart(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("vector-retrieve", fixtureRAGTool)

	retrieveTool := ancora.NewToolSpec("vector-retrieve", "retrieves relevant chunks")
	spec := ancora.NewAgentSpecBuilder().
		WithName("rag-agent").
		WithModelID("llama3").
		WithTool(retrieveTool).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestRAGStoreRecordsRetrievalEvent(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("rag-run-1")
	_ = store.AppendEvent("rag-run-1", 0, `{"type":"activity_recorded","activity_kind":"retrieval","key":"retrieve-1"}`)

	events, _ := store.EventsForRun("rag-run-1")
	if len(events) != 1 {
		t.Fatalf("expected 1 stored retrieval event, got: %d", len(events))
	}
}
