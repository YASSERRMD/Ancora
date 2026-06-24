// multi-agent-verifier runs an agent and a verifier concurrently,
// demonstrating how two independent runs can be tracked in parallel.
package main

import (
	"context"
	"fmt"
	"os"
	"sync"

	"ancora.io/sdk/ancora"
)

type runResult struct {
	name   string
	id     string
	events []string
	err    error
}

func runAgent(ctx context.Context, tr ancora.Transport, name, model, instructions string) runResult {
	spec := ancora.NewAgentSpec(name, model, instructions)
	ag := ancora.NewTransportAgent(tr, spec)
	run, err := ag.Start(ctx)
	if err != nil {
		return runResult{name: name, err: err}
	}
	evs, err := run.DrainEvents(ctx)
	return runResult{name: name, id: run.ID(), events: evs, err: err}
}

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)

	var wg sync.WaitGroup
	results := make([]runResult, 2)
	wg.Add(2)

	go func() {
		defer wg.Done()
		results[0] = runAgent(context.Background(), tr,
			"main-agent", "llama3", "you are the primary agent")
	}()
	go func() {
		defer wg.Done()
		results[1] = runAgent(context.Background(), tr,
			"verifier-agent", "llama3", "verify the primary agent output")
	}()
	wg.Wait()

	for _, r := range results {
		if r.err != nil {
			fmt.Fprintf(os.Stderr, "%s error: %v\n", r.name, r.err)
			os.Exit(1)
		}
		fmt.Printf("%s run_id=%s events=%d\n", r.name, r.id, len(r.events))
		for _, ev := range r.events {
			fmt.Printf("  %s\n", ev)
		}
	}
}
