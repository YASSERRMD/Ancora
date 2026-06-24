// ancora-agent is a single-binary offline Ancora agent runner.
// It reads an AgentSpec from stdin (protobuf bytes) or --spec flag,
// runs the agent via the in-process FFI runtime, and prints events to stdout.
package main

import (
	"context"
	"flag"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
)

func main() {
	name := flag.String("name", "agent", "agent name")
	model := flag.String("model", "llama3", "model ID")
	instructions := flag.String("instructions", "you are a helpful assistant", "agent instructions")
	flag.Parse()

	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "ancora-agent: runtime init: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	spec := ancora.NewAgentSpec(*name, *model, *instructions)
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "ancora-agent: start: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("run_id=%s\n", run.ID())

	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "ancora-agent: drain: %v\n", err)
		os.Exit(1)
	}
	for _, ev := range evs {
		fmt.Println(ev)
	}
}
