// structured-output shows how to derive a JSON Schema from a Go struct
// and embed it in an agent system prompt before starting the run.
package main

import (
	"context"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
)

// AnalysisResult is the shape we want the agent to emit.
type AnalysisResult struct {
	Summary    string   `json:"summary"    schema:"One-sentence summary of the analysis"`
	Topics     []string `json:"topics"     schema:"List of main topics identified"`
	Confidence float64  `json:"confidence" schema:"Confidence score between 0 and 1"`
	ActionItem string   `json:"action_item" schema:"Recommended next action"`
}

// ClassificationResult is a second struct to show schema generation for
// a different output shape using the same helper.
type ClassificationResult struct {
	Label      string `json:"label"      schema:"Primary classification label"`
	Subcategory string `json:"subcategory" schema:"More specific subcategory"`
	Score      int    `json:"score"      schema:"Integer confidence 0-100"`
}

func main() {
	schema, err := ancora.SchemaFromStruct(AnalysisResult{})
	if err != nil {
		fmt.Fprintf(os.Stderr, "schema: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("schema: %s\n", schema)

	classSchema, err := ancora.SchemaFromStruct(ClassificationResult{})
	if err != nil {
		fmt.Fprintf(os.Stderr, "class schema: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("classification schema: %s\n", classSchema)

	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	system := fmt.Sprintf(
		"You are an analysis agent. Always respond with valid JSON matching this schema:\n%s",
		schema,
	)
	spec := ancora.NewAgentSpec("analysis-agent", "llama3", system)
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
	fmt.Printf("received %d event(s)\n", len(evs))
	fmt.Println("done")
}
