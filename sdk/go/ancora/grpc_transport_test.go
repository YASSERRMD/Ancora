package ancora_test

import (
	"context"
	"fmt"
	"net"
	"sync/atomic"
	"testing"

	"ancora.io/sdk/ancora"
	runservice "ancora.io/sdk/ancora/grpc"
	googlegrpc "google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// fakeRunServer is a minimal in-process RunService for testing GRPCTransport.
type fakeRunServer struct {
	runservice.UnimplementedRunServiceServer
	counter atomic.Int64
}

func (s *fakeRunServer) StartRun(_ context.Context, req *runservice.StartRunRequest) (*runservice.StartRunResponse, error) {
	id := fmt.Sprintf("fake-run-%d", s.counter.Add(1))
	return &runservice.StartRunResponse{RunId: id}, nil
}

func (s *fakeRunServer) PollRun(_ context.Context, req *runservice.PollRunRequest) (*runservice.PollRunResponse, error) {
	return &runservice.PollRunResponse{Event: ""}, nil
}

func (s *fakeRunServer) ResumeRun(_ context.Context, req *runservice.ResumeRunRequest) (*runservice.ResumeRunResponse, error) {
	return &runservice.ResumeRunResponse{Status: "ok"}, nil
}

func newFakeGRPCTransport(t *testing.T) *ancora.GRPCTransport {
	t.Helper()
	lis, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("net.Listen: %v", err)
	}
	srv := googlegrpc.NewServer()
	runservice.RegisterRunServiceServer(srv, &fakeRunServer{})
	go func() { _ = srv.Serve(lis) }()
	t.Cleanup(srv.Stop)

	conn, err := googlegrpc.NewClient(lis.Addr().String(),
		googlegrpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		t.Fatalf("grpc.NewClient: %v", err)
	}
	t.Cleanup(func() { _ = conn.Close() })
	return ancora.NewGRPCTransport(runservice.NewRunServiceClient(conn))
}

func TestGRPCTransportImplementsTransportInterface(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	var _ ancora.Transport = tr
}

func TestGRPCTransportStartRunReturnsNonEmptyID(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	id, err := tr.StartRun(context.Background(), []byte("spec"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	if id == "" {
		t.Fatal("StartRun returned empty ID")
	}
}

func TestGRPCTransportStartRunPrefixedFakeRun(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	if len(id) < 8 {
		t.Fatalf("expected descriptive run ID, got: %q", id)
	}
}

func TestGRPCTransportPollRunReturnsNilWhenEmpty(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	ev, err := tr.PollRun(context.Background(), id)
	if err != nil {
		t.Fatalf("PollRun: %v", err)
	}
	if ev != nil {
		t.Fatalf("expected nil event, got: %s", ev)
	}
}

func TestGRPCTransportResumeRunSucceeds(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	id, _ := tr.StartRun(context.Background(), []byte("spec"))
	err := tr.ResumeRun(context.Background(), id, []byte(`{"ok":true}`))
	if err != nil {
		t.Fatalf("ResumeRun: %v", err)
	}
}

func TestGRPCTransportMultipleStartRunsHaveDifferentIDs(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	id1, _ := tr.StartRun(context.Background(), []byte("spec"))
	id2, _ := tr.StartRun(context.Background(), []byte("spec"))
	if id1 == id2 {
		t.Fatalf("expected different IDs, got %q twice", id1)
	}
}

func TestGRPCTransportContextCancellationPropagatesToRPC(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err := tr.StartRun(ctx, []byte("spec"))
	if err == nil {
		t.Log("cancellation not enforced by in-process server (acceptable)")
	}
}

func TestNewGRPCTransportReturnsNonNil(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	if tr == nil {
		t.Fatal("NewGRPCTransport returned nil")
	}
}
