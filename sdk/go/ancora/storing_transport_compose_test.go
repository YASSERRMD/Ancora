package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

// TestStoringTransportComposability verifies that two SqliteStores can each
// wrap the same inner transport and both record the same run ID.
func TestStoringTransportComposability(t *testing.T) {
	s1 := mustInMemoryStore(t)
	s2 := mustInMemoryStore(t)
	inner := newFakeGRPCTransport(t)
	tr1 := ancora.NewStoringTransport(inner, s1)
	tr2 := ancora.NewStoringTransport(inner, s2)

	id1, err := tr1.StartRun(context.Background(), []byte("spec"))
	if err != nil {
		t.Fatalf("tr1.StartRun: %v", err)
	}
	id2, err := tr2.StartRun(context.Background(), []byte("spec"))
	if err != nil {
		t.Fatalf("tr2.StartRun: %v", err)
	}

	ok1, _ := s1.HasRun(id1)
	ok2, _ := s2.HasRun(id2)
	if !ok1 || !ok2 {
		t.Fatal("both stores must record their respective run IDs")
	}
}

// TestStoringTransportChained verifies that a StoringTransport can wrap
// another StoringTransport (double-layer recording).
func TestStoringTransportChained(t *testing.T) {
	s1 := mustInMemoryStore(t)
	s2 := mustInMemoryStore(t)
	inner := ancora.NewStoringTransport(newFakeGRPCTransport(t), s1)
	outer := ancora.NewStoringTransport(inner, s2)

	id, err := outer.StartRun(context.Background(), []byte("spec"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	ok1, _ := s1.HasRun(id)
	ok2, _ := s2.HasRun(id)
	if !ok1 {
		t.Fatal("inner store must record the run ID")
	}
	if !ok2 {
		t.Fatal("outer store must record the run ID")
	}
}

func TestStoringTransportPollRunChainedStoredInBothLayers(t *testing.T) {
	s1 := mustInMemoryStore(t)
	s2 := mustInMemoryStore(t)
	inner := ancora.NewStoringTransport(newEventGRPCTransport(t, []string{"ev"}), s1)
	outer := ancora.NewStoringTransport(inner, s2)

	id, _ := outer.StartRun(context.Background(), []byte("spec"))
	outer.PollRun(context.Background(), id)

	n1, _ := s1.EventCount(id)
	n2, _ := s2.EventCount(id)
	if n1 != 1 {
		t.Fatalf("inner layer: expected 1 event, got %d", n1)
	}
	if n2 != 1 {
		t.Fatalf("outer layer: expected 1 event, got %d", n2)
	}
}
