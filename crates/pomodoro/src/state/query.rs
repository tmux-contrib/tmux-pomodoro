use crate::state::model::{FromRow, Session, SessionEvent};
use anyhow::{Context, Result};
use regex::Regex;
use rusqlite::{named_params, Connection, Transaction, TransactionBehavior};
use std::collections::HashMap;
use std::sync::LazyLock;
use uuid::Uuid;

/// DATABASE_SCHEMA for the database, embedded at compile time from `schema.sql`.
const DATABASE_SCHEMA: &str = include_str!("schema.sql");

/// Named SQL queries parsed from the embedded `query.sql` file.
///
/// Populated once on first access. Each query in `query.sql` is delimited by
/// a `-- name: <key>` header and a trailing `--` sentinel, for example:
///
/// ```sql
/// -- name: get_session
/// SELECT * FROM sessions WHERE session_id = :session_id
/// --
/// ```
///
/// The map key is the trimmed name string (e.g. `"get_session"`).
/// Look up a query with `DATABASE_QUERY.get("query_name")`.
static DATABASE_QUERY: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    // Matches blocks of the form:
    //   -- name: <name>\n<sql body>\n--
    // Capture 1 = name, Capture 2 = SQL body (multiline via `(?s)`).
    let regexp = Regex::new(r"(?s)--\s*name:\s*([^\n]+)\n(.*?)\n--").expect("Invalid regex");
    // DATABASE_QUERY_RAW file.
    const DATABASE_QUERY_RAW: &str = include_str!("query.sql");

    let mut queries = HashMap::new();
    // Load the SQL queries into a hash map.
    for captures in regexp.captures_iter(DATABASE_QUERY_RAW) {
        let name = captures[1].trim().to_string();
        let query = captures[2].trim().to_string();
        queries.insert(name, query);
    }
    queries
});

/// Database manages the SQLite connection lifecycle: opening, migrating, and
/// vending [`Querier`] handles for executing queries.
///
/// Every write operation should go through [`Database::transaction`] so that
/// partial failures roll back automatically.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open a connection to the SQLite database.
    pub fn open() -> Result<Self> {
        let path = xdg::BaseDirectories::with_prefix("pomodoro")
            .place_state_file("state.db")
            .context("Failed to determine database path")?;
        let conn = Connection::open(path).context("Failed to open database connection")?;
        Ok(Self { conn })
    }

    /// Open a connection to the in-memory SQLite database.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to open database connection")?;
        Ok(Self { conn })
    }

    /// Return a reference to the underlying connection.
    ///
    /// Intended for tests that need to construct a [`Querier`] directly from
    /// the connection (e.g. to seed data before running a command under test).
    #[cfg(test)]
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Begin a SQLite transaction and return it.
    ///
    /// Pass `&*tx` (or rely on deref coercion with `&tx`) to [`Querier::new`] to
    /// execute queries within the transaction. The caller must call
    /// [`Transaction::commit`] explicitly; dropping without committing rolls back.
    pub fn transaction(&mut self) -> Result<Transaction<'_>> {
        self.conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .context("Failed to start transaction")
    }

    /// Apply the embedded SQL schema, creating all tables if they do not already exist.
    ///
    /// Safe to call on an existing database — the schema uses `CREATE TABLE IF NOT EXISTS`
    /// semantics. Must be called once after opening before any queries are executed.
    pub fn migrate(&self) -> Result<()> {
        self.conn
            .execute_batch(DATABASE_SCHEMA)
            .context("Failed to migrate database")
    }
}

/// Querier executes SQL queries against a borrowed [`Connection`].
///
/// The lifetime `'q` is the lifetime of the underlying connection or transaction.
/// Construct one via [`Querier::new`], passing either a plain `&Connection` or
/// `&*transaction` (possible because [`Transaction`] derefs to [`Connection`]).
pub struct Querier<'q> {
    conn: &'q Connection,
}

impl<'q> Querier<'q> {
    /// Create a [`Querier`] from a borrowed connection or transaction.
    ///
    /// The typical production usage passes a reference to an active
    /// [`Transaction`], relying on the deref coercion from `Transaction` to
    /// `Connection`:
    ///
    /// ```ignore
    /// let tx = database.transaction()?;
    /// let querier = Querier::new(&tx);
    /// // ... execute queries ...
    /// tx.commit()?;
    /// ```
    pub fn new(conn: &'q Connection) -> Self {
        Self { conn }
    }

    /// Insert a new session row and return the persisted [`Session`].
    pub fn insert_session(&self, args: &InsertSessionArgs) -> Result<Session> {
        let query = DATABASE_QUERY
            .get("insert_session")
            .context("Failed to get query")?;

        let mut operation = self
            .conn
            .prepare(query)
            .context("Failed to prepare query")?;

        let session = operation
            .query_one(
                named_params! {
                    ":session_id": args.session.id,
                    ":session_kind": args.session.kind,
                    ":planned_secs": args.session.planned_duration.num_seconds(),
                    ":created_at": args.session.created_at,
                },
                Session::from_row,
            )
            .context("Failed to execute query")?;

        Ok(session)
    }

    /// Retrieve a single [`Session`] by its UUID, returning an error if not found.
    pub fn get_session_by_id(&self, args: &GetSessionByIdArgs) -> Result<Session> {
        let query = DATABASE_QUERY
            .get("get_session")
            .context("Failed to get query")?;

        let mut operation = self
            .conn
            .prepare(query)
            .context("Failed to prepare query")?;

        let session = operation
            .query_one(
                named_params! {
                    ":session_id": args.session_id,
                },
                Session::from_row,
            )
            .context("Failed to execute query")?;

        Ok(session)
    }

    /// Retrieve a paginated list of sessions ordered by `session_id DESC` (newest first).
    pub fn list_sessions(&self, args: &ListSessionsArgs) -> Result<Vec<Session>> {
        let query = DATABASE_QUERY
            .get("list_sessions")
            .context("Failed to get query")?;

        let mut operation = self
            .conn
            .prepare(query)
            .context("Failed to prepare query")?;

        let iterator = operation
            .query_map(
                named_params! {
                    ":limit": args.limit,
                    ":offset": args.offset,
                },
                Session::from_row,
            )
            .context("Failed to execute query")?;

        let mut collection = Vec::new();
        for item in iterator {
            let session = item.context("Failed to map query result")?;
            collection.push(session);
        }

        Ok(collection)
    }

    /// Insert a new session event row and return the persisted [`SessionEvent`].
    pub fn insert_session_event(&self, args: &InsertSessionEventArgs) -> Result<SessionEvent> {
        let query = DATABASE_QUERY
            .get("insert_session_event")
            .context("Failed to get query")?;

        let mut operation = self
            .conn
            .prepare(query)
            .context("Failed to prepare query")?;

        let session_event = operation
            .query_one(
                named_params! {
                    ":session_event_id": args.session_event.id,
                    ":session_event_kind": args.session_event.kind,
                    ":session_id": args.session_event.session_id,
                    ":created_at": args.session_event.created_at,
                },
                SessionEvent::from_row,
            )
            .context("Failed to execute query")?;

        Ok(session_event)
    }

    /// Retrieve a single [`SessionEvent`] by its UUID, returning an error if not found.
    #[cfg(test)]
    pub fn get_session_event_by_id(&self, args: &GetSessionEventByIdArgs) -> Result<SessionEvent> {
        let query = DATABASE_QUERY
            .get("get_session_event")
            .context("Failed to get query")?;

        let mut operation = self
            .conn
            .prepare(query)
            .context("Failed to prepare query")?;

        let session_event = operation
            .query_one(
                named_params! {
                    ":session_event_id": args.session_event_id,
                },
                SessionEvent::from_row,
            )
            .context("Failed to execute query")?;

        Ok(session_event)
    }

    /// Retrieve a paginated list of session events ordered by `session_event_id DESC` (newest first).
    pub fn list_session_events(&self, args: &ListSessionEventsArgs) -> Result<Vec<SessionEvent>> {
        let query = DATABASE_QUERY
            .get("list_session_events")
            .context("Failed to get query")?;

        let mut operation = self
            .conn
            .prepare(query)
            .context("Failed to prepare query")?;

        let iterator = operation
            .query_map(
                named_params! {
                    ":session_id": args.session_id,
                    ":limit": args.limit,
                    ":offset": args.offset,
                },
                SessionEvent::from_row,
            )
            .context("Failed to execute query")?;

        let mut collection = Vec::new();
        for item in iterator {
            let session = item.context("Failed to map query result")?;
            collection.push(session);
        }

        Ok(collection)
    }
}

/// Arguments for [`Querier::insert_session`].
#[derive(Debug)]
pub struct InsertSessionArgs<'s> {
    /// The session to persist.
    pub session: &'s Session,
}

/// Arguments for [`Querier::get_session_by_id`].
#[derive(Debug)]
pub struct GetSessionByIdArgs<'u> {
    /// The UUID of the session to look up.
    pub session_id: &'u Uuid,
}

/// Arguments for [`Querier::list_sessions`].
#[derive(Debug)]
pub struct ListSessionsArgs {
    /// Maximum number of rows to return.
    pub limit: Option<u32>,
    /// Number of rows to skip before returning results.
    pub offset: Option<u32>,
}

impl ListSessionEventsArgs {
    /// Returns args that fetch only the single most recent session event.
    ///
    /// Equivalent to `LIMIT 1` with no offset, ordered by `session_event_id DESC`.
    /// Use this when you only need the current state of the session (i.e. the
    /// latest event in the log).
    pub fn first() -> Self {
        Self {
            session_id: None,
            limit: Some(1),
            offset: None,
        }
    }

    /// Returns args that fetch the complete event history for a single session.
    ///
    /// Equivalent to `WHERE session_id = <id>` with no `LIMIT` or offset,
    /// ordered by `session_event_id DESC` (most recent event first).
    /// Use this when you need every event recorded against a known session.
    pub fn with_session_id(session_id: Uuid) -> Self {
        Self {
            session_id: Some(session_id),
            limit: Some(u32::MAX),
            offset: None,
        }
    }
}

impl ListSessionsArgs {
    /// Returns args that fetch only the single most recent session.
    ///
    /// Equivalent to `LIMIT 1` with no offset, ordered by `session_id DESC`.
    /// Use this when you only need the latest session record.
    pub fn first() -> Self {
        Self {
            limit: Some(1),
            offset: None,
        }
    }
}

/// Returns args with no limit and no offset, fetching all sessions.
impl Default for ListSessionsArgs {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
        }
    }
}

/// Arguments for [`Querier::insert_session_event`].
#[derive(Debug)]
pub struct InsertSessionEventArgs<'e> {
    /// The session_event to persist.
    pub session_event: &'e SessionEvent,
}

/// Arguments for [`Querier::get_session_event_by_id`].
#[cfg(test)]
#[derive(Debug)]
pub struct GetSessionEventByIdArgs<'u> {
    /// The UUID of the session_event to look up.
    pub session_event_id: &'u Uuid,
}

/// Arguments for [`Querier::list_session_events`].
#[derive(Debug)]
pub struct ListSessionEventsArgs {
    /// Restrict results to events belonging to this session; `None` returns events for all sessions.
    pub session_id: Option<Uuid>,
    /// Maximum number of rows to return.
    pub limit: Option<u32>,
    /// Number of rows to skip before returning results.
    pub offset: Option<u32>,
}

/// Returns the most recent session event (limit 1, offset 0).
impl Default for ListSessionEventsArgs {
    fn default() -> Self {
        Self {
            session_id: None,
            limit: Some(1),
            offset: Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Open an in-memory database, apply the schema, and return it.
    ///
    /// Used by every test in this module as the starting point for a clean,
    /// isolated database that is discarded when the test completes.
    fn setup() -> Result<Database> {
        let database = Database::open_in_memory().context("Failed to open database")?;
        database.migrate().context("Failed to migrate database")?;
        Ok(database)
    }

    #[test]
    fn insert_session_returns_persisted_session() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &InsertSessionArgs { session };
        let session = querier.insert_session(args)?;
        assert_eq!(
            args.session, &session,
            "Inserted session should match the input session"
        );

        Ok(())
    }

    #[test]
    fn get_session_by_id_returns_matching_session() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &InsertSessionArgs { session };
        let session = querier.insert_session(args)?;

        let args = &GetSessionByIdArgs {
            session_id: &session.id,
        };
        let session = querier.get_session_by_id(args)?;
        assert_eq!(
            args.session_id, &session.id,
            "Retrieved session ID should match the requested ID"
        );

        Ok(())
    }

    #[test]
    fn get_session_by_id_fails_when_not_found() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &GetSessionByIdArgs {
            session_id: &session.id,
        };
        let result = querier.get_session_by_id(args);
        assert!(
            result.is_err(),
            "Should return error when session not found"
        );

        Ok(())
    }

    #[test]
    fn list_sessions_returns_inserted_session() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &InsertSessionArgs { session };
        let session = querier.insert_session(args)?;

        let args = &ListSessionsArgs::first();
        let result = querier.list_sessions(args)?;
        assert_eq!(
            result.len(),
            1,
            "Should return exactly one session after single insert"
        );
        assert_eq!(
            &session, &result[0],
            "Listed session should match the created session"
        );

        Ok(())
    }

    #[test]
    fn insert_session_event_returns_persisted_event() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &InsertSessionArgs { session };
        let session = querier.insert_session(args)?;

        let session_event = &SessionEvent {
            session_id: session.id,
            ..SessionEvent::default()
        };
        let args = &InsertSessionEventArgs { session_event };
        let session_event = querier.insert_session_event(args)?;
        assert_eq!(
            args.session_event, &session_event,
            "Inserted session event should match the input session event"
        );

        Ok(())
    }

    #[test]
    fn get_session_event_by_id_returns_matching_event() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &InsertSessionArgs { session };
        querier.insert_session(args)?;

        let session_event = &SessionEvent {
            session_id: session.id,
            ..SessionEvent::default()
        };
        let args = &InsertSessionEventArgs { session_event };
        let session_event = querier.insert_session_event(args)?;

        let args = &GetSessionEventByIdArgs {
            session_event_id: &session_event.id,
        };
        let session_event = querier.get_session_event_by_id(args)?;
        assert_eq!(
            args.session_event_id, &session_event.id,
            "Retrieved session ID should match the requested ID"
        );

        Ok(())
    }

    #[test]
    fn get_session_event_by_id_fails_when_not_found() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session_event = &SessionEvent::default();
        let args = &GetSessionEventByIdArgs {
            session_event_id: &session_event.id,
        };
        let result = querier.get_session_event_by_id(args);
        assert!(
            result.is_err(),
            "Should return error when session event not found"
        );

        Ok(())
    }

    #[test]
    fn list_session_events_returns_inserted_event() -> Result<()> {
        let database = setup()?;
        let querier = Querier::new(database.connection());

        let session = &Session::default();
        let args = &InsertSessionArgs { session };
        querier.insert_session(args)?;

        let session_event = &SessionEvent {
            session_id: session.id,
            ..SessionEvent::default()
        };
        let args = &InsertSessionEventArgs { session_event };
        let session_event = querier.insert_session_event(args)?;

        let args = &ListSessionEventsArgs::first();
        let result = querier.list_session_events(args)?;
        assert_eq!(
            result.len(),
            1,
            "Should return exactly one session event after single insert"
        );
        assert_eq!(
            &session_event, &result[0],
            "Listed session event should match the created session event"
        );

        Ok(())
    }
}
