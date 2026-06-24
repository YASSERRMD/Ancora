// grpc-transport demonstrates connecting to a remote Ancora gRPC server
// and running an agent via GRPCTransport. The server address is read from
// ANCORA_GRPC_ADDR (default "localhost:50051").
package main

import (
	"context"
	"fmt"
	"os"

	"ancora.io/sdk/ancora"
	runservice "ancora.io/sdk/ancora/grpc"
	googlegrpc "google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	addr := os.Getenv("ANCORA_GRPC_ADDR")
	if addr == "" {
		addr = "localhost:50051"
	}

	conn, err := googlegrpc.NewClient(addr, googlegrpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		fmt.Fprintf(os.Stderr, "dial %s: %v\n", addr, err)
		os.Exit(1)
	}
	defer conn.Close()

	client := runservice.NewRunServiceClient(conn)
	tr := ancora.NewGRPCTransport(client)

	spec := ancora.NewAgentSpec("grpc-agent", "llama3", "run via grpc")
	ag := ancora.NewTransportAgent(tr, spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "start: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("run started: %s\n", run.ID())

	events, err := run.DrainEvents(context.Background())
	if err != nil {
		fmt.Fprintf(os.Stderr, "drain: %v\n", err)
		os.Exit(1)
	}
	for i, ev := range events {
		fmt.Printf("  event[%d]: %s\n", i+1, ev)
	}
	fmt.Println("done")
}
