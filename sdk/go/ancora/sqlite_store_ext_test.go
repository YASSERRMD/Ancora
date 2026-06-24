package ancora_test

import (
	"os"
	"path/filepath"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestSqliteStoreEventCountZeroInitially(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("r")
	n, err := s.EventCount("r")
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if n != 0 {
		t.Fatalf("expected 0, got: %d", n)
	}
}

func TestSqliteStoreEventCountGrowsWithAppend(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("r")
	s.AppendEvent("r", 0, "e0")
	s.AppendEvent("r", 1, "e1")
	n, err := s.EventCount("r")
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if n != 2 {
		t.Fatalf("expected 2, got: %d", n)
	}
}

func TestSqliteStoreRunCountInitiallyZero(t *testing.T) {
	s := mustInMemoryStore(t)
	n, err := s.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if n != 0 {
		t.Fatalf("expected 0, got: %d", n)
	}
}

func TestSqliteStoreRunCountIncrementsOnRecord(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("a")
	s.RecordRun("b")
	n, _ := s.RunCount()
	if n != 2 {
		t.Fatalf("expected 2, got: %d", n)
	}
}

func TestSqliteStoreDeleteRunRemovesRun(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("del")
	s.AppendEvent("del", 0, "e")
	if err := s.DeleteRun("del"); err != nil {
		t.Fatalf("DeleteRun: %v", err)
	}
	ok, _ := s.HasRun("del")
	if ok {
		t.Fatal("HasRun must be false after DeleteRun")
	}
}

func TestSqliteStoreDeleteRunClearsEvents(t *testing.T) {
	s := mustInMemoryStore(t)
	s.RecordRun("del2")
	s.AppendEvent("del2", 0, "x")
	s.DeleteRun("del2")
	evs, _ := s.EventsForRun("del2")
	if len(evs) != 0 {
		t.Fatalf("events must be deleted with run, got: %d", len(evs))
	}
}

func TestSqliteStoreFileBasedPersists(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "test.db")
	{
		s, err := ancora.OpenSqliteStore(path)
		if err != nil {
			t.Fatalf("open: %v", err)
		}
		s.RecordRun("persistent-run")
		s.Close()
	}
	_, err := os.Stat(path)
	if err != nil {
		t.Fatalf("db file must exist after close: %v", err)
	}
	{
		s, err := ancora.OpenSqliteStore(path)
		if err != nil {
			t.Fatalf("reopen: %v", err)
		}
		defer s.Close()
		ok, _ := s.HasRun("persistent-run")
		if !ok {
			t.Fatal("run must persist across open/close cycles")
		}
	}
}
