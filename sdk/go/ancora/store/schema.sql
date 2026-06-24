CREATE TABLE IF NOT EXISTS runs (
    id TEXT PRIMARY KEY,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS run_events (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id  TEXT NOT NULL REFERENCES runs(id),
    seq     INTEGER NOT NULL,
    payload TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_run_events_run_id ON run_events(run_id, seq);
