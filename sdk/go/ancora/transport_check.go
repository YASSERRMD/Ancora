package ancora

// Compile-time assertions that all concrete transports satisfy Transport.
var _ Transport = (*CgoTransport)(nil)
var _ Transport = (*GRPCTransport)(nil)
