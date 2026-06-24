// Package ancora provides the Go SDK for the Ancora agent runtime.
//
// The central concept is a Transport: anything that can start a run, poll for
// events, and resume a suspended run. Three implementations are included:
//
//   - [CgoTransport]: embeds the Rust runtime via CGO. No network required.
//   - [GRPCTransport]: forwards calls to a remote gRPC RunService.
//   - [StoringTransport]: decorates any Transport with SQLite event persistence.
//
// A typical local workflow looks like:
//
//	rt, _ := ancora.NewRuntime()
//	defer rt.Free()
//	spec := ancora.NewAgentSpec("my-agent", "llama3", "do the thing")
//	agent := ancora.NewTransportAgent(ancora.NewCgoTransport(rt), spec)
//	run, _ := agent.Start(ctx)
//	for ev := range run.EventChan(ctx) {
//	    fmt.Println(string(ev))
//	}
//
// To run against a remote server, swap [NewCgoTransport] for [NewGRPCTransport]:
//
//	conn, _ := grpc.NewClient(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
//	client := runservice.NewRunServiceClient(conn)
//	tr := ancora.NewGRPCTransport(client)
//
// [NewTransportFromEnv] selects the transport automatically based on the
// ANCORA_TRANSPORT environment variable ("cgo" or "grpc").
//
// To verify that a transport behaves correctly, use [NewConformanceSuite]:
//
//	suite := ancora.NewConformanceSuite(tr)
//	results := suite.RunAll(ctx)
package ancora
