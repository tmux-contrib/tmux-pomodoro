-- sqlfluff:dialect:sqlite
-- sqlfluff:max_line_length:1024
-- sqlfluff:rules:capitalisation.keywords:capitalisation_policy:upper

-- Session represents a single pomodoro session, which has a unique ID, a type
-- (e.g., "focus" or "break"), a duration in seconds, and a timestamp for when
-- it was created. The session_id is the primary key, and the planned_secs must
-- be greater than 0.
CREATE TABLE IF NOT EXISTS session (
    session_id TEXT PRIMARY KEY,
    session_kind TEXT NOT NULL,
    planned_secs INTEGER NOT NULL CHECK (planned_secs > 0),
    created_at INTEGER NOT NULL
);

-- Session events are used to track the state of a session, such as when it
-- starts, is paused, or ends. This allows us to reconstruct the session's
-- history and determine its current state.
CREATE TABLE IF NOT EXISTS session_event (
    session_event_id TEXT PRIMARY KEY,
    session_event_kind TEXT NOT NULL,
    session_id TEXT NOT NULL REFERENCES session (session_id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL
);
