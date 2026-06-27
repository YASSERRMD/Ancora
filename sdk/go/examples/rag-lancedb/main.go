// rag-lancedb builds an offline RAG context from a small document corpus
// and injects it into an agent system prompt before starting the run.
// No LanceDB daemon is required -- the retrieval is fully in-memory.
package main

import (
	"context"
	"fmt"
	"os"
	"strings"

	"ancora.io/sdk/ancora"
)

// ragCorpus is a tiny in-process document store used as a stand-in for
// a real LanceDB table. Each entry is a (source, passage) pair.
type ragCorpus struct {
	passages []struct{ source, text string }
}

func (c *ragCorpus) add(source, text string) {
	c.passages = append(c.passages, struct{ source, text string }{source, text})
}

// retrieve does a naive keyword overlap search -- illustrates the
// retrieval-injection pattern without requiring a live vector store.
func (c *ragCorpus) retrieve(query string, topK int) []string {
	type scored struct {
		text  string
		score int
	}
	words := strings.Fields(strings.ToLower(query))
	var results []scored
	for _, p := range c.passages {
		score := 0
		lower := strings.ToLower(p.text)
		for _, w := range words {
			if strings.Contains(lower, w) {
				score++
			}
		}
		if score > 0 {
			results = append(results, scored{fmt.Sprintf("[%s] %s", p.source, p.text), score})
		}
	}
	// sort descending by score
	for i := 0; i < len(results)-1; i++ {
		for j := i + 1; j < len(results); j++ {
			if results[j].score > results[i].score {
				results[i], results[j] = results[j], results[i]
			}
		}
	}
	out := make([]string, 0, topK)
	for _, r := range results {
		if len(out) >= topK {
			break
		}
		out = append(out, r.text)
	}
	return out
}

func main() {
	corpus := &ragCorpus{}
	corpus.add("docs/overview.md", "Ancora is a multi-backend agent runtime for Rust and Go.")
	corpus.add("docs/backends.md", "Supported backends include pgvector, qdrant, weaviate, and lancedb.")
	corpus.add("docs/embeddings.md", "The embedders module provides offline hash-based and TF-IDF embedders.")
	corpus.add("docs/retrieval.md", "RetrievalPipeline performs cosine similarity search over stored passages.")

	query := "what backends does ancora support"
	hits := corpus.retrieve(query, 3)
	context_ := strings.Join(hits, "\n---\n")
	fmt.Printf("retrieved %d passage(s)\n", len(hits))

	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	system := fmt.Sprintf(
		"You are a helpful assistant. Use only the context below to answer questions.\n\nCONTEXT:\n%s",
		context_,
	)
	spec := ancora.NewAgentSpec("rag-agent", "llama3", system)
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("run started: %s\n", run.ID())

	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("done: %d event(s)\n", len(evs))
}
