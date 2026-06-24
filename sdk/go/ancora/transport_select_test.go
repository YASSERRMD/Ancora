package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func TestNewTransportFromEnvDefaultsCgoTransport(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	t.Setenv("ANCORA_TRANSPORT", "")
	tr, err := ancora.NewTransportFromEnv(rt)
	if err != nil {
		t.Fatalf("NewTransportFromEnv cgo: %v", err)
	}
	if tr == nil {
		t.Fatal("expected non-nil transport")
	}
}

func TestNewTransportFromEnvCgoKeyword(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	t.Setenv("ANCORA_TRANSPORT", "cgo")
	tr, err := ancora.NewTransportFromEnv(rt)
	if err != nil {
		t.Fatalf("NewTransportFromEnv cgo keyword: %v", err)
	}
	if tr == nil {
		t.Fatal("expected non-nil transport")
	}
}

func TestNewTransportFromEnvCgoRequiresRuntime(t *testing.T) {
	t.Setenv("ANCORA_TRANSPORT", "cgo")
	_, err := ancora.NewTransportFromEnv(nil)
	if err == nil {
		t.Fatal("expected error when Runtime is nil for cgo transport")
	}
}

func TestNewTransportFromEnvUnknownValueReturnsError(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	t.Setenv("ANCORA_TRANSPORT", "unknown-backend")
	_, err := ancora.NewTransportFromEnv(rt)
	if err == nil {
		t.Fatal("expected error for unknown ANCORA_TRANSPORT value")
	}
}

func TestNewTransportFromEnvGRPCConnectsToAddress(t *testing.T) {
	t.Setenv("ANCORA_TRANSPORT", "grpc")
	t.Setenv("ANCORA_GRPC_ADDR", "localhost:19999")
	tr, err := ancora.NewTransportFromEnv(nil)
	if err != nil {
		t.Fatalf("NewTransportFromEnv grpc: %v", err)
	}
	if tr == nil {
		t.Fatal("expected non-nil gRPC transport")
	}
}
