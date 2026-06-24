package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestSqliteStoreListRunsEmpty(t *testing.T) {
	s := mustInMemoryStore(t)
	ids, err := s.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	if len(ids) != 0 {
		t.Fatalf("expected 0 runs, got: %d", len(ids))
	}
}

func TestSqliteStoreListRunsReturnsRecordedIDs(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("r1")
	s.RecordRun("r2")
	s.RecordRun("r3")
	ids, err := s.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	if len(ids) != 3 {
		t.Fatalf("expected 3 run IDs, got: %d", len(ids))
	}
}

func TestSqliteStoreListRunsDoesNotIncludeDeleted(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("keep")
	s.RecordRun("gone")
	s.DeleteRun("gone")
	ids, err := s.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	for _, id := range ids {
		if id == "gone" {
			t.Fatal("deleted run must not appear in ListRuns")
		}
	}
	if len(ids) != 1 || ids[0] != "keep" {
		t.Fatalf("expected [keep], got: %v", ids)
	}
}

func TestStoringTransportListRunsViaStore(t *testing.T) {
	s := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(newFakeGRPCTransport(t), s)
	tr.StartRun(context.Background(), []byte("spec"))
	tr.StartRun(context.Background(), []byte("spec"))
	ids, err := s.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	if len(ids) != 2 {
		t.Fatalf("expected 2 stored runs, got: %d", len(ids))
	}
}
