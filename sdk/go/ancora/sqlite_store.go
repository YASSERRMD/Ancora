package ancora

import (
	"database/sql"
	_ "embed"
	"fmt"

	_ "modernc.org/sqlite"
)

//go:embed store/schema.sql
var schemaSQL string

// SqliteStore persists run IDs and events to a local SQLite database.
type SqliteStore struct {
	db *sql.DB
}

// OpenSqliteStore opens (or creates) a SQLite database at path and applies
// the schema. Use ":memory:" for an in-process store.
func OpenSqliteStore(path string) (*SqliteStore, error) {
	db, err := sql.Open("sqlite", path)
	if err != nil {
		return nil, fmt.Errorf("ancora: open sqlite %s: %w", path, err)
	}
	if _, err := db.Exec(schemaSQL); err != nil {
		db.Close()
		return nil, fmt.Errorf("ancora: apply schema: %w", err)
	}
	return &SqliteStore{db: db}, nil
}

// Close releases the database connection.
func (s *SqliteStore) Close() error { return s.db.Close() }

// RecordRun inserts a run ID into the runs table.
func (s *SqliteStore) RecordRun(runID string) error {
	_, err := s.db.Exec(`INSERT OR IGNORE INTO runs(id) VALUES(?)`, runID)
	return err
}

// AppendEvent appends a serialized event payload for runID at the given seq.
func (s *SqliteStore) AppendEvent(runID string, seq int, payload string) error {
	_, err := s.db.Exec(
		`INSERT INTO run_events(run_id, seq, payload) VALUES(?, ?, ?)`,
		runID, seq, payload,
	)
	return err
}

// EventsForRun returns all event payloads for runID in seq order.
func (s *SqliteStore) EventsForRun(runID string) ([]string, error) {
	rows, err := s.db.Query(
		`SELECT payload FROM run_events WHERE run_id = ? ORDER BY seq`,
		runID,
	)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var out []string
	for rows.Next() {
		var p string
		if err := rows.Scan(&p); err != nil {
			return nil, err
		}
		out = append(out, p)
	}
	return out, rows.Err()
}

// HasRun returns true if runID exists in the runs table.
func (s *SqliteStore) HasRun(runID string) (bool, error) {
	var n int
	err := s.db.QueryRow(`SELECT COUNT(*) FROM runs WHERE id = ?`, runID).Scan(&n)
	return n > 0, err
}
