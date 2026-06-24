package ancora

import (
	"context"

	"ancora.io/sdk/ancora/grpc"
)

// GRPCTransport implements Transport via a remote gRPC RunService.
type GRPCTransport struct {
	client runservice.RunServiceClient
}

// NewGRPCTransport wraps a RunServiceClient in a GRPCTransport.
func NewGRPCTransport(client runservice.RunServiceClient) *GRPCTransport {
	return &GRPCTransport{client: client}
}

// StartRun starts a new agent run using the gRPC RunService.
func (t *GRPCTransport) StartRun(ctx context.Context, spec []byte) (string, error) {
	resp, err := t.client.StartRun(ctx, &runservice.StartRunRequest{AgentSpec: spec})
	if err != nil {
		return "", err
	}
	return resp.GetRunId(), nil
}

// PollRun pops the next event for runID via the gRPC RunService.
// Returns nil bytes when no event is available.
func (t *GRPCTransport) PollRun(ctx context.Context, runID string) ([]byte, error) {
	resp, err := t.client.PollRun(ctx, &runservice.PollRunRequest{RunId: runID})
	if err != nil {
		return nil, err
	}
	ev := resp.GetEvent()
	if ev == "" {
		return nil, nil
	}
	return []byte(ev), nil
}

// ResumeRun provides a human decision to a suspended run via gRPC.
func (t *GRPCTransport) ResumeRun(ctx context.Context, runID string, decision []byte) error {
	_, err := t.client.ResumeRun(ctx, &runservice.ResumeRunRequest{
		RunId:    runID,
		Decision: decision,
	})
	return err
}
