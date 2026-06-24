package ancora_test

import (
	"context"
	"net"
	"testing"

	runservice "ancora.io/sdk/ancora/grpc"
	googlegrpc "google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"

	"ancora.io/sdk/ancora"
)

// errorRunServer returns gRPC status errors for all RPCs.
type errorRunServer struct {
	runservice.UnimplementedRunServiceServer
}

func (s *errorRunServer) StartRun(_ context.Context, _ *runservice.StartRunRequest) (*runservice.StartRunResponse, error) {
	return nil, status.Error(codes.Internal, "forced start error")
}

func (s *errorRunServer) PollRun(_ context.Context, _ *runservice.PollRunRequest) (*runservice.PollRunResponse, error) {
	return nil, status.Error(codes.NotFound, "run not found")
}

func (s *errorRunServer) ResumeRun(_ context.Context, _ *runservice.ResumeRunRequest) (*runservice.ResumeRunResponse, error) {
	return nil, status.Error(codes.FailedPrecondition, "run not suspended")
}

func newErrorGRPCTransport(t *testing.T) *ancora.GRPCTransport {
	t.Helper()
	lis, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("net.Listen: %v", err)
	}
	srv := googlegrpc.NewServer()
	runservice.RegisterRunServiceServer(srv, &errorRunServer{})
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

func TestGRPCTransportStartRunPropagatesServerError(t *testing.T) {
	tr := newErrorGRPCTransport(t)
	_, err := tr.StartRun(context.Background(), []byte("spec"))
	if err == nil {
		t.Fatal("expected error from server, got nil")
	}
}

func TestGRPCTransportPollRunPropagatesServerError(t *testing.T) {
	tr := newErrorGRPCTransport(t)
	_, err := tr.PollRun(context.Background(), "fake-id")
	if err == nil {
		t.Fatal("expected error from server, got nil")
	}
}

func TestGRPCTransportResumeRunPropagatesServerError(t *testing.T) {
	tr := newErrorGRPCTransport(t)
	err := tr.ResumeRun(context.Background(), "fake-id", []byte(`{}`))
	if err == nil {
		t.Fatal("expected error from server, got nil")
	}
}
