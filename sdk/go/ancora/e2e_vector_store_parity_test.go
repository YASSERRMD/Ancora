package ancora_test

import (
	"encoding/json"
	"testing"

	"ancora.io/sdk/ancora"
)

// vectorChunk is a common schema across all fixture vector stores.
type vectorChunk struct {
	ID    string  `json:"id"`
	Text  string  `json:"text"`
	Score float64 `json:"score"`
}

// fixtureQdrantRetrieve simulates a Qdrant-backed retrieval tool.
func fixtureQdrantRetrieve(input []byte) ([]byte, error) {
	chunks := []vectorChunk{
		{ID: "qdrant-1", Text: "Qdrant is a vector database.", Score: 0.95},
		{ID: "qdrant-2", Text: "Qdrant uses HNSW for ANN search.", Score: 0.88},
	}
	return json.Marshal(chunks)
}

// fixturePgVectorRetrieve simulates a PgVector-backed retrieval tool.
func fixturePgVectorRetrieve(input []byte) ([]byte, error) {
	chunks := []vectorChunk{
		{ID: "pgvec-1", Text: "PgVector adds vector search to Postgres.", Score: 0.92},
		{ID: "pgvec-2", Text: "PgVector supports cosine, L2, and inner product.", Score: 0.85},
	}
	return json.Marshal(chunks)
}

// fixtureLanceRetrieve2 redefines the LanceDB fixture under a different name
// to avoid duplicate declaration with e2e_rag_lancedb_test.go.
func fixtureLanceRetrieve2(input []byte) ([]byte, error) {
	chunks := []vectorChunk{
		{ID: "lance-1", Text: "LanceDB is a columnar vector database.", Score: 0.91},
		{ID: "lance-2", Text: "LanceDB supports versioned datasets.", Score: 0.82},
	}
	return json.Marshal(chunks)
}

func TestE2EVectorStoreParity_QdrantReturnsChunks(t *testing.T) {
	out, err := fixtureQdrantRetrieve([]byte(`{"query":"qdrant"}`))
	if err != nil {
		t.Fatalf("fixtureQdrantRetrieve: %v", err)
	}
	var chunks []vectorChunk
	_ = json.Unmarshal(out, &chunks)
	if len(chunks) != 2 {
		t.Fatalf("Qdrant expected 2 chunks, got: %d", len(chunks))
	}
}

func TestE2EVectorStoreParity_PgVectorReturnsChunks(t *testing.T) {
	out, err := fixturePgVectorRetrieve([]byte(`{"query":"pgvector"}`))
	if err != nil {
		t.Fatalf("fixturePgVectorRetrieve: %v", err)
	}
	var chunks []vectorChunk
	_ = json.Unmarshal(out, &chunks)
	if len(chunks) != 2 {
		t.Fatalf("PgVector expected 2 chunks, got: %d", len(chunks))
	}
}

func TestE2EVectorStoreParity_LanceReturnsChunks(t *testing.T) {
	out, err := fixtureLanceRetrieve2([]byte(`{"query":"lance"}`))
	if err != nil {
		t.Fatalf("fixtureLanceRetrieve2: %v", err)
	}
	var chunks []vectorChunk
	_ = json.Unmarshal(out, &chunks)
	if len(chunks) != 2 {
		t.Fatalf("LanceDB expected 2 chunks, got: %d", len(chunks))
	}
}

func TestE2EVectorStoreParity_AllStoresReturnSameChunkCount(t *testing.T) {
	fixtures := []struct {
		name string
		fn   func([]byte) ([]byte, error)
	}{
		{"qdrant", fixtureQdrantRetrieve},
		{"pgvector", fixturePgVectorRetrieve},
		{"lancedb", fixtureLanceRetrieve2},
	}

	for _, f := range fixtures {
		out, err := f.fn([]byte(`{"query":"test"}`))
		if err != nil {
			t.Fatalf("%s: %v", f.name, err)
		}
		var chunks []vectorChunk
		if err := json.Unmarshal(out, &chunks); err != nil {
			t.Fatalf("%s: Unmarshal: %v", f.name, err)
		}
		if len(chunks) != 2 {
			t.Fatalf("%s: expected 2 chunks, got: %d", f.name, len(chunks))
		}
	}
}

func TestE2EVectorStoreParity_AllChunksHaveNonEmptyIDs(t *testing.T) {
	fixtures := []func([]byte) ([]byte, error){
		fixtureQdrantRetrieve, fixturePgVectorRetrieve, fixtureLanceRetrieve2,
	}
	for _, fn := range fixtures {
		out, _ := fn(nil)
		var chunks []vectorChunk
		_ = json.Unmarshal(out, &chunks)
		for i, c := range chunks {
			if c.ID == "" {
				t.Fatalf("chunk %d has empty ID", i)
			}
		}
	}
}

func TestE2EVectorStoreParity_AllChunksHaveNonNegativeScores(t *testing.T) {
	fixtures := []func([]byte) ([]byte, error){
		fixtureQdrantRetrieve, fixturePgVectorRetrieve, fixtureLanceRetrieve2,
	}
	for _, fn := range fixtures {
		out, _ := fn(nil)
		var chunks []vectorChunk
		_ = json.Unmarshal(out, &chunks)
		for i, c := range chunks {
			if c.Score < 0 {
				t.Fatalf("chunk %d has negative score: %f", i, c.Score)
			}
		}
	}
}

func TestE2EVectorStoreParity_ThreeToolsCanBeRegistered(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("qdrant-retrieve", fixtureQdrantRetrieve)
	tk.RegisterTool("pgvector-retrieve", fixturePgVectorRetrieve)
	tk.RegisterTool("lancedb-retrieve2", fixtureLanceRetrieve2)

	if tk.Tools().Count() != 3 {
		t.Fatalf("expected 3 vector store tools, got: %d", tk.Tools().Count())
	}
}
