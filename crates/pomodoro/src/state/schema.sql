CREATE TABLE IF NOT EXISTS session (
  session_id   TEXT PRIMARY KEY,
  session_type TEXT NOT NULL,
  duration_sec INTEGER NOT NULL CHECK (duration_sec > 0),
  created_at   INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS session_event (
  session_event_id   INTEGER PRIMARY KEY AUTOINCREMENT,
  session_event_type TEXT NOT NULL,
  session_id         TEXT NOT NULL REFERENCES Session(session_id) ON DELETE CASCADE,
  created_at         INTEGER NOT NULL
);
