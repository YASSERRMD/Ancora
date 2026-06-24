package ancora

import (
	"fmt"
	"os"

	"ancora.io/sdk/ancora/grpc"
	googlegrpc "google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// NewTransportFromEnv selects a Transport based on ANCORA_TRANSPORT env var.
// "grpc" connects to ANCORA_GRPC_ADDR (default "localhost:50051").
// All other values (including unset) use the in-process CGo transport.
func NewTransportFromEnv(rt *Runtime) (Transport, error) {
	switch os.Getenv("ANCORA_TRANSPORT") {
	case "grpc":
		addr := os.Getenv("ANCORA_GRPC_ADDR")
		if addr == "" {
			addr = "localhost:50051"
		}
		return newGRPCTransportFromAddr(addr)
	case "", "cgo":
		if rt == nil {
			return nil, fmt.Errorf("ancora: cgo transport requires a non-nil Runtime")
		}
		return NewCgoTransport(rt), nil
	default:
		return nil, fmt.Errorf("ancora: unknown ANCORA_TRANSPORT %q", os.Getenv("ANCORA_TRANSPORT"))
	}
}

// newGRPCTransportFromAddr dials addr and returns a GRPCTransport.
func newGRPCTransportFromAddr(addr string) (*GRPCTransport, error) {
	conn, err := googlegrpc.NewClient(addr, googlegrpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, fmt.Errorf("ancora: grpc dial %s: %w", addr, err)
	}
	return NewGRPCTransport(runservice.NewRunServiceClient(conn)), nil
}
