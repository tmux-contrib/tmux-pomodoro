-- sqlfluff:dialect:sqlite
-- sqlfluff:max_line_length:1024
-- sqlfluff:rules:capitalisation.keywords:capitalisation_policy:upper

-- name: insert_session
INSERT INTO session (
    session_id,
    session_kind,
    planned_secs,
    created_at
)
VALUES (
    :session_id,
    :session_kind,
    :planned_secs,
    :created_at
)
RETURNING *;
--

-- name: get_session
SELECT
    session_id,
    session_kind,
    planned_secs,
    created_at
FROM session
WHERE
    session_id = :session_id;
--

-- name: list_sessions
SELECT
    session_id,
    session_kind,
    planned_secs,
    created_at
FROM session
ORDER BY session_id DESC
LIMIT COALESCE(:limit, -1) OFFSET COALESCE(:offset, 0);
--

-- name: insert_session_event
INSERT INTO session_event (
    session_event_id,
    session_event_kind,
    session_id,
    created_at
)
VALUES (
    :session_event_id,
    :session_event_kind,
    :session_id,
    :created_at
)
RETURNING *;
--

-- name: get_session_event
SELECT
    session_event_id,
    session_event_kind,
    session_id,
    created_at
FROM session_event
WHERE
    session_event_id = :session_event_id;
--

-- name: list_session_events
SELECT
    session_event_id,
    session_event_kind,
    session_id,
    created_at
FROM session_event
WHERE
    (:session_id IS NULL OR session_id = :session_id)
ORDER BY session_event_id DESC
LIMIT COALESCE(:limit, -1) OFFSET COALESCE(:offset, 0);
--
