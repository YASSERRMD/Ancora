package ancora_test

import (
	"context"
	"net"
	"testing"

	"ancora.io/sdk/ancora"
	runservice "ancora.io/sdk/ancora/grpc"
	googlegrpc "google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// eventRunServer returns one event then empties.
type eventRunServer struct {
	runservice.UnimplementedRunServiceServer
	events []string
}

func (s *eventRunServer) StartRun(_ context.Context, _ *runservice.StartRunRequest) (*runservice.StartRunResponse, error) {
	return &runservice.StartRunResponse{RunId: "event-run-1"}, nil
}

func (s *eventRunServer) PollRun(_ context.Context, _ *runservice.PollRunRequest) (*runservice.PollRunResponse, error) {
	if len(s.events) == 0 {
		return &runservice.PollRunResponse{Event: ""}, nil
	}
	ev := s.events[0]
	s.events = s.events[1:]
	return &runservice.PollRunResponse{Event: ev}, nil
}

func (s *eventRunServer) ResumeRun(_ context.Context, _ *runservice.ResumeRunRequest) (*runservice.ResumeRunResponse, error) {
	return &runservice.ResumeRunResponse{Status: "ok"}, nil
}

func newEventGRPCTransport(t *testing.T, events []string) *ancora.GRPCTransport {
	t.Helper()
	lis, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("net.Listen: %v", err)
	}
	srv := googlegrpc.NewServer()
	runservice.RegisterRunServiceServer(srv, &eventRunServer{events: events})
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

func TestTransportRunDrainEventsReturnsAllEvents(t *testing.T) {
	events := []string{"e1", "e2", "e3"}
	tr := newEventGRPCTransport(t, events)
	spec := ancora.NewAgentSpec("drain-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	got, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(got) != 3 {
		t.Fatalf("expected 3 events, got: %d", len(got))
	}
}

func TestTransportRunDrainEventsEmptyReturnsNil(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("empty-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	got, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(got) != 0 {
		t.Fatalf("expected 0 events, got: %d", len(got))
	}
}

func TestTransportRunEventChanClosesWhenEmpty(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	spec := ancora.NewAgentSpec("chan-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	ch := run.EventChan(context.Background())
	var count int
	for range ch {
		count++
	}
	if count != 0 {
		t.Fatalf("expected 0 events on channel, got: %d", count)
	}
}

func TestTransportRunEventChanDeliversEvents(t *testing.T) {
	events := []string{"ev1", "ev2"}
	tr := newEventGRPCTransport(t, events)
	spec := ancora.NewAgentSpec("deliver-agent", "llama3", "hi")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	ch := run.EventChan(context.Background())
	var got []string
	for ev := range ch {
		got = append(got, string(ev))
	}
	if len(got) != 2 {
		t.Fatalf("expected 2 events, got: %d", len(got))
	}
}

func TestTransportRunCgoBackendDrainEvents(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-drain", "llama3", "hello")
	run, err := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	_ = evs
}
