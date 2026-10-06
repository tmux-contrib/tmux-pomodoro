use chrono::{DateTime, Duration, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

/// A trait for mapping a database [`Row`] to a concrete type.
///
/// Implement this trait for any struct that should be constructable directly
/// from a SQLite query result row.
pub trait FromRow {
    /// Map the columns of `row` to `Self`, returning a [`rusqlite::Error`] on
    /// any type mismatch or missing column.
    fn from_row(row: &Row) -> rusqlite::Result<Self>
    where
        Self: Sized;
}

/// The type of a pomodoro session — either a focus session or a break.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SessionKind {
    /// Focus mode is the default session type for the pomodoro timer, where users focus on their
    /// tasks.
    Focus,

    /// Break mode is a session type for the pomodoro timer that allows users to take a short or
    /// long break.
    Break,
}

impl Display for SessionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Focus => write!(f, "focus"),
            Self::Break => write!(f, "break"),
        }
    }
}

impl TryFrom<&str> for SessionKind {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "focus" => Ok(Self::Focus),
            "break" => Ok(Self::Break),
            other => Err(format!("unknown session kind: {other}")),
        }
    }
}

impl rusqlite::types::FromSql for SessionKind {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str()?;
        SessionKind::try_from(value).map_err(|e| rusqlite::types::FromSqlError::Other(e.into()))
    }
}

impl rusqlite::types::ToSql for SessionKind {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

/// A single timed pomodoro session — either a focus or break interval.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Session {
    /// Unique identifier for the session.
    pub id: Uuid,
    /// Whether this is a focus or break session.
    pub kind: SessionKind,
    /// Planned duration of the session.
    #[serde(
        rename = "planned_secs",
        serialize_with = "serialize_duration_as_secs",
        deserialize_with = "deserialize_duration_from_secs"
    )]
    pub planned_duration: Duration,
    /// Timestamp when the session was created.
    pub created_at: DateTime<Utc>,
}

/// Returns a 25-minute (1500 s) focus session with a freshly generated ID and the current time.
impl Default for Session {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            kind: SessionKind::Focus,
            planned_duration: Duration::seconds(1500),
            created_at: Utc::now(),
        }
    }
}

impl FromRow for Session {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("session_id")?,
            kind: row.get("session_kind")?,
            planned_duration: Duration::seconds(row.get("planned_secs")?),
            created_at: row.get("created_at")?,
        })
    }
}

/// The kind of event recorded against a [`Session`], representing each transition
/// in the session state machine.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SessionEventKind {
    /// Indicates that the session has started running.
    ///
    /// This must be the first event for every session.
    /// After this event, the session is considered to be in the
    /// `running` state until a `paused`, `completed`, or `aborted`
    /// event is recorded.
    Started,

    /// Indicates that a previously paused session has resumed running.
    ///
    /// This event is only valid when the session is currently
    /// in the `paused` state. After this event, the session
    /// transitions back to the `running` state and elapsed time
    /// accumulation continues.
    Resumed,

    /// Indicates that the session has been paused by the user.
    ///
    /// This event is only valid when the session is currently
    /// in the `running` state. After this event, the session
    /// transitions to the `paused` state and elapsed time
    /// accumulation stops.
    Paused,

    /// Indicates that the session was aborted before reaching its
    /// planned duration.
    ///
    /// This event is typically recorded when the user explicitly
    /// resets or cancels the session. It is a terminal event.
    /// After this event, no further events may be recorded for
    /// the session.
    Aborted,

    /// Indicates that the session completed naturally by reaching
    /// its planned duration.
    ///
    /// This is a terminal event. After this event, the session
    /// transitions to the `completed` state and no further events
    /// may be recorded.
    Completed,
}

impl Display for SessionEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Started => write!(f, "started"),
            Self::Resumed => write!(f, "resumed"),
            Self::Paused => write!(f, "paused"),
            Self::Aborted => write!(f, "aborted"),
            Self::Completed => write!(f, "completed"),
        }
    }
}

impl TryFrom<&str> for SessionEventKind {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "started" => Ok(Self::Started),
            "resumed" => Ok(Self::Resumed),
            "paused" => Ok(Self::Paused),
            "aborted" => Ok(Self::Aborted),
            "completed" => Ok(Self::Completed),
            other => Err(format!("unknown session event kind: {other}")),
        }
    }
}

impl rusqlite::types::FromSql for SessionEventKind {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let value = value.as_str()?;
        SessionEventKind::try_from(value)
            .map_err(|e| rusqlite::types::FromSqlError::Other(e.into()))
    }
}

impl rusqlite::types::ToSql for SessionEventKind {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

/// An event recorded against a [`Session`], representing a single state transition.
///
/// Events are emitted when a session is started, paused, resumed, aborted, or completed.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SessionEvent {
    /// Unique identifier for the event (UUID v7).
    pub id: Uuid,
    /// The type of event (started, paused, resumed, aborted, completed).
    pub kind: SessionEventKind,
    /// Foreign key referencing the parent session.
    pub session_id: Uuid,
    /// Timestamp when the event was recorded.
    pub created_at: DateTime<Utc>,
}

/// Returns a [`SessionEventKind::Started`] event with a freshly generated ID,
/// a nil session ID, and the current timestamp.
impl Default for SessionEvent {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            kind: SessionEventKind::Started,
            session_id: Uuid::default(),
            created_at: Utc::now(),
        }
    }
}

impl FromRow for SessionEvent {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("session_event_id")?,
            kind: row.get("session_event_kind")?,
            session_id: row.get("session_id")?,
            created_at: row.get("created_at")?,
        })
    }
}

impl SessionEvent {
    /// Creates a [`SessionEventKind::Started`] event for the given session.
    ///
    /// Use this when a new session begins for the first time.
    pub fn started(session_id: Uuid) -> Self {
        Self {
            session_id,
            ..Self::default()
        }
    }

    /// Creates a [`SessionEventKind::Paused`] event for the given session.
    ///
    /// Use this when the user pauses a currently running session.
    pub fn paused(session_id: Uuid) -> Self {
        Self {
            session_id,
            kind: SessionEventKind::Paused,
            ..Self::default()
        }
    }

    /// Creates a [`SessionEventKind::Resumed`] event for the given session.
    ///
    /// Use this when the user resumes a previously paused session.
    pub fn resumed(session_id: Uuid) -> Self {
        Self {
            session_id,
            kind: SessionEventKind::Resumed,
            ..Self::default()
        }
    }

    /// Creates a [`SessionEventKind::Aborted`] event for the given session.
    ///
    /// Use this when the user cancels a session before it reaches its planned duration.
    pub fn aborted(session_id: Uuid) -> Self {
        Self {
            session_id,
            kind: SessionEventKind::Aborted,
            ..Self::default()
        }
    }

    /// Creates a [`SessionEventKind::Completed`] event for the given session.
    ///
    /// Use this when a session naturally reaches its planned duration.
    pub fn completed(session_id: Uuid) -> Self {
        Self {
            session_id,
            kind: SessionEventKind::Completed,
            ..Self::default()
        }
    }
}

fn serialize_duration_as_secs<S>(d: &Duration, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_i64(d.num_seconds())
}

fn deserialize_duration_from_secs<'de, D>(d: D) -> std::result::Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let secs = i64::deserialize(d)?;
    Ok(Duration::seconds(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_kind_try_from_unknown_returns_error() {
        let result = SessionKind::try_from("unknown");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "unknown session kind: unknown");
    }

    #[test]
    fn session_event_kind_try_from_unknown_returns_error() {
        let result = SessionEventKind::try_from("unknown");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "unknown session event kind: unknown");
    }
}
