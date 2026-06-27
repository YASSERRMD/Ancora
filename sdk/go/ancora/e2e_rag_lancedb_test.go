package ancora_test

import (
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// lanceChunk mirrors a LanceDB-like retrieval result for fixture testing.
type lanceChunk struct {
	ID    string  `json:"id"`
	Text  string  `json:"text"`
	Score float64 `json:"_distance"`
}

func fixtureLanceRetrieve(input []byte) ([]byte, error) {
	chunks := []lanceChunk{
		{ID: "doc-1", Text: "Rust is a systems programming language.", Score: 0.05},
		{ID: "doc-2", Text: "Rust achieves memory safety without GC.", Score: 0.12},
		{ID: "doc-3", Text: "Cargo is the Rust package manager.", Score: 0.18},
	}
	return json.Marshal(chunks)
}

func TestE2ERAGLanceDBToolReturnsThreeChunks(t *testing.T) {
	out, err := fixtureLanceRetrieve([]byte(`{"query":"rust memory","top_k":3}`))
	if err != nil {
		t.Fatalf("fixtureLanceRetrieve: %v", err)
	}
	var chunks []lanceChunk
	if err := json.Unmarshal(out, &chunks); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if len(chunks) != 3 {
		t.Fatalf("expected 3 chunks, got: %d", len(chunks))
	}
}

func TestE2ERAGLanceDBChunksHaveNonEmptyText(t *testing.T) {
	out, _ := fixtureLanceRetrieve(nil)
	var chunks []lanceChunk
	_ = json.Unmarshal(out, &chunks)
	for i, c := range chunks {
		if c.Text == "" {
			t.Fatalf("chunk %d has empty text", i)
		}
	}
}

func TestE2ERAGLanceDBChunksHaveNonEmptyIDs(t *testing.T) {
	out, _ := fixtureLanceRetrieve(nil)
	var chunks []lanceChunk
	_ = json.Unmarshal(out, &chunks)
	for i, c := range chunks {
		if c.ID == "" {
			t.Fatalf("chunk %d has empty ID", i)
		}
	}
}

func TestE2ERAGLanceDBChunkDistancesAscend(t *testing.T) {
	out, _ := fixtureLanceRetrieve(nil)
	var chunks []lanceChunk
	_ = json.Unmarshal(out, &chunks)
	for i := 1; i < len(chunks); i++ {
		if chunks[i].Score < chunks[i-1].Score {
			t.Fatalf("LanceDB distances must ascend (lower=closer), got chunk[%d]=%f < chunk[%d]=%f",
				i, chunks[i].Score, i-1, chunks[i-1].Score)
		}
	}
}

func TestE2ERAGLanceDBToolRegisteredInToolkit(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("lancedb-retrieve", fixtureLanceRetrieve)
	if !tk.Tools().Has("lancedb-retrieve") {
		t.Fatal("toolkit must have lancedb-retrieve after registration")
	}
}

func TestE2ERAGLanceDBAgentSpecIncludesRetrieverTool(t *testing.T) {
	tool := ancora.NewToolSpec("lancedb-retrieve", "Retrieves nearest chunks from LanceDB")
	spec := ancora.NewAgentSpecBuilder().
		WithName("rag-lancedb-agent").
		WithModelID("gpt-4o").
		WithTool(tool).
		Build()
	if len(spec.GetTools()) != 1 {
		t.Fatalf("expected 1 tool, got: %d", len(spec.GetTools()))
	}
}

func TestE2ERAGLanceDBRunWithRetrieverStarts(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	tool := ancora.NewToolSpec("lancedb-retrieve", "Retrieves nearest chunks from LanceDB")
	spec := ancora.NewAgentSpecBuilder().
		WithName("rag-lancedb-agent").
		WithModelID("llama3").
		WithTool(tool).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestE2ERAGLanceDBOutputContainsRustText(t *testing.T) {
	out, _ := fixtureLanceRetrieve([]byte(`{}`))
	if !strings.Contains(string(out), "Rust") {
		t.Fatalf("output must contain 'Rust', got: %s", out)
	}
}

func TestE2ERAGLanceDBStoreRecordsRetrievalEvent(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("lance-rag-1")
	_ = store.AppendEvent("lance-rag-1", 0, `{"type":"activity_recorded","kind":"retrieval","store":"lancedb"}`)

	events, _ := store.EventsForRun("lance-rag-1")
	if len(events) != 1 {
		t.Fatalf("expected 1 RAG event, got: %d", len(events))
	}
}

func TestE2ERAGLanceDBRegistryInvokeReturnsParseable(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("lancedb-retrieve", fixtureLanceRetrieve)

	out, err := reg.Invoke("lancedb-retrieve", []byte(`{"query":"rust","top_k":3}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	var chunks []lanceChunk
	if err := json.Unmarshal(out, &chunks); err != nil {
		t.Fatalf("Unmarshal registry output: %v", err)
	}
	if len(chunks) != 3 {
		t.Fatalf("expected 3 chunks, got: %d", len(chunks))
	}
}
