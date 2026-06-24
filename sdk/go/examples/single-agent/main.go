// single-agent runs a single agent to completion and prints its events.
package main

import (
	"context"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
)

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	spec := ancora.NewAgentSpec("single-agent", "llama3", "you are a helpful assistant")
	ag := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("started run: %s\n", run.ID())

	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain: %v\n", err)
		os.Exit(1)
	}
	for _, ev := range evs {
		fmt.Println(ev)
	}
	fmt.Println("done")
}
