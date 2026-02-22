use crate::app::cli::*;
use crate::hook::run::{Runner, SessionEventArgs};
use crate::state::model::*;
use crate::state::query::*;
use anyhow::Result;
use chrono::{Duration, Utc};
use minijinja::Environment;
use uuid::Uuid;

/// Converts [`StartCommandArgs`] into a [`Session`], applying default durations when none
/// are provided (25 minutes for focus, 5 minutes for break).
impl From<&StartCommandArgs> for Session {
    fn from(value: &StartCommandArgs) -> Self {
        let config = ProgramConfig::default();
        let duration = value.duration.unwrap_or(match value.mode {
            StartMode::Focus => config.focus_duration,
            StartMode::Break => config.break_duration,
        });
        Session {
            kind: value.mode.into(),
            planned_duration: Duration::seconds(duration.as_secs() as i64),
            ..Session::default()
        }
    }
}

/// Converts a CLI [`StartMode`] into the equivalent [`SessionKind`] stored in the database.
impl From<StartMode> for SessionKind {
    fn from(value: StartMode) -> Self {
        match value {
            StartMode::Focus => SessionKind::Focus,
            StartMode::Break => SessionKind::Break,
        }
    }
}

/// StartCommand is responsible for starting a new pomodoro timer session.
pub struct StartCommand<'q> {
    /// Runner is used to execute the hooks.
    pub runner: Option<Runner>,
    /// Querier is used to retrieve the current status of the pomodoro timer from the database.
    pub querier: Querier<'q>,
}

impl<'q> StartCommand<'q> {
    /// Execute the StartCommand with the provided arguments.
    pub fn execute(&self, args: &StartCommandArgs) -> Result<()> {
        let params = ListSessionEventsArgs::first();
        let result = self.querier.list_session_events(&params)?;

        let mut session: Session;
        let session_event = match result.first() {
            None => {
                session = Session::from(args);
                session = self.insert_session(&session)?;
                println!("Started a new {} session.", session.kind);
                Some(SessionEvent::started(session.id))
            }
            Some(session_event) => match session_event.kind {
                SessionEventKind::Started | SessionEventKind::Resumed => {
                    session = self.get_session(&session_event.session_id)?;
                    println!("A {} session is already running.", session.kind);
                    None
                }
                SessionEventKind::Aborted | SessionEventKind::Completed => {
                    session = Session::from(args);
                    session = self.insert_session(&session)?;
                    println!("Started a new {} session.", session.kind);
                    Some(SessionEvent::started(session.id))
                }
                SessionEventKind::Paused => {
                    session = self.get_session(&session_event.session_id)?;
                    println!("Resumed the {} session.", session.kind);
                    Some(SessionEvent::resumed(session.id))
                }
            },
        };

        if let Some(session_event) = session_event.as_ref() {
            let params = InsertSessionEventArgs { session_event };
            self.querier.insert_session_event(&params)?;

            if let Some(runner) = &self.runner {
                let args = SessionEventArgs {
                    session: session.clone(),
                    session_event: session_event.clone(),
                };
                // execute the hook
                runner.execute(&args).ok();
            }
        }

        Ok(())
    }

    /// Retrieve an existing [`Session`] by its UUID.
    fn get_session(&self, session_id: &Uuid) -> Result<Session> {
        let params = GetSessionByIdArgs { session_id };
        let session = self.querier.get_session_by_id(&params)?;
        Ok(session)
    }

    /// Persist a new [`Session`] and return the stored record.
    fn insert_session(&self, session: &Session) -> Result<Session> {
        let params = InsertSessionArgs { session };
        let session = self.querier.insert_session(&params)?;
        Ok(session)
    }
}

/// StopCommand is responsible for stopping the current pomodoro timer session. It can also reset
/// the session entirely when the `--reset` flag is provided.
pub struct StopCommand<'q> {
    /// Runner is used to execute the hooks.
    pub runner: Option<Runner>,
    /// Querier is used to retrieve the current status of the pomodoro timer from the database.
    pub querier: Querier<'q>,
}

impl<'q> StopCommand<'q> {
    /// Execute the StopCommand with the provided arguments.
    pub fn execute(&self, args: &StopCommandArgs) -> Result<()> {
        let params = ListSessionEventsArgs::first();
        let result = self.querier.list_session_events(&params)?;

        let mut session: Session = Session::default();
        let session_event = match result.first() {
            Some(session_event) => match session_event.kind {
                SessionEventKind::Started | SessionEventKind::Resumed => {
                    session = self.get_session(&session_event.session_id)?;
                    if args.reset {
                        println!("Aborted the {} session.", session.kind);
                        Some(SessionEvent::aborted(session.id))
                    } else {
                        println!("Paused the {} session.", session.kind);
                        Some(SessionEvent::paused(session.id))
                    }
                }
                SessionEventKind::Paused => {
                    session = self.get_session(&session_event.session_id)?;
                    if args.reset {
                        println!("Aborted the {} session.", session.kind);
                        Some(SessionEvent::aborted(session.id))
                    } else {
                        println!("The {} session is already paused.", session.kind);
                        None
                    }
                }
                SessionEventKind::Aborted | SessionEventKind::Completed => {
                    session = self.get_session(&session_event.session_id)?;
                    println!("No active {} session to stop.", session.kind);
                    None
                }
            },
            None => {
                println!("No active session found.");
                None
            }
        };

        if let Some(session_event) = session_event.as_ref() {
            let params = InsertSessionEventArgs { session_event };
            self.querier.insert_session_event(&params)?;

            if let Some(runner) = &self.runner {
                let args = SessionEventArgs {
                    session: session.clone(),
                    session_event: session_event.clone(),
                };
                // execute the hook
                runner.execute(&args).ok();
            }
        }

        Ok(())
    }

    /// Retrieve an existing [`Session`] by its UUID.
    fn get_session(&self, session_id: &Uuid) -> Result<Session> {
        let params = GetSessionByIdArgs { session_id };
        let session = self.querier.get_session_by_id(&params)?;
        Ok(session)
    }
}

/// The lifecycle state of the most recent session.
#[derive(Default, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionState {
    /// No session exists yet.
    #[default]
    None,
    /// The session is actively counting down.
    Running,
    /// The session has been paused by the user.
    Paused,
    /// The session reached its planned duration.
    Completed,
    /// The session was cancelled before finishing.
    Aborted,
}

impl From<&SessionEventKind> for SessionState {
    fn from(kind: &SessionEventKind) -> Self {
        match kind {
            SessionEventKind::Started | SessionEventKind::Resumed => Self::Running,
            SessionEventKind::Paused => Self::Paused,
            SessionEventKind::Completed => Self::Completed,
            SessionEventKind::Aborted => Self::Aborted,
        }
    }
}

/// SessionStatus holds the computed fields for the current session, used as the
/// data model for both JSON and text output of the `status` command.
#[derive(serde::Serialize)]
pub struct SessionStatus {
    /// The session kind: `"focus"`, `"break"`, or `"none"`.
    pub kind: String,
    /// The lifecycle state of the session.
    pub state: SessionState,
    /// Planned duration of the session in seconds.
    pub planned_secs: i64,
    /// Total elapsed time in seconds.
    pub elapsed_secs: i64,
    /// Remaining time in seconds (clamped to zero).
    pub remaining_secs: i64,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self {
            kind: "none".to_string(),
            state: Default::default(),
            planned_secs: Default::default(),
            elapsed_secs: Default::default(),
            remaining_secs: Default::default(),
        }
    }
}
/// StatusCommand computes and displays the current status of the most recent
/// pomodoro session. It calculates elapsed and remaining time by replaying the
/// session event log, auto-inserts a [`SessionEventKind::Completed`] event when
/// a running session has no time left, and renders the result via
/// [`StatusCommand::render`].
pub struct StatusCommand<'q> {
    /// Runner is used to execute the hooks.
    pub runner: Option<Runner>,
    /// Querier is used to retrieve the current status of the pomodoro timer from the database.
    pub querier: Querier<'q>,
}

impl<'q> StatusCommand<'q> {
    /// Compute the current [`SessionStatus`] and render it to stdout.
    ///
    /// 1. Fetches the most recent session and its full event log.
    /// 2. Replays events in chronological order to accumulate elapsed time.
    /// 3. Derives the current [`SessionState`] from the most recent event.
    /// 4. Auto-completes the session (inserts a `Completed` event) when the
    ///    session is still `Running` but has no remaining time.
    /// 5. Delegates formatting to [`StatusCommand::render`].
    pub fn execute(&self, args: &StatusCommandArgs) -> Result<()> {
        let params = &ListSessionsArgs::first();
        let result = self.querier.list_sessions(params)?;

        match result.first() {
            Some(session) => {
                let params = &ListSessionEventsArgs::with_session_id(session.id);
                let result = self.querier.list_session_events(params)?;

                let mut session_started_at = None;
                let mut session_elapsed_time = Duration::zero();

                for session_event in result.iter().rev() {
                    let kind = &session_event.kind;
                    // Find the start and end of each range
                    if matches!(kind, SessionEventKind::Started | SessionEventKind::Resumed) {
                        session_started_at = Some(session_event.created_at);
                    } else if let Some(since_start) = session_started_at.take() {
                        session_elapsed_time += session_event.created_at - since_start;
                    }
                }

                if let Some(since_start) = session_started_at {
                    session_elapsed_time += Utc::now() - since_start;
                }

                // prepare the session kind
                let session_kind = session.kind.to_string();

                // Determine the session state from the last event
                let session_state = result
                    .first()
                    .map(|e| SessionState::from(&e.kind))
                    .unwrap_or_default();

                // Calculate the different duration types
                let session_planned_secs = session.planned_duration.num_seconds();
                let session_elapsed_secs = session_elapsed_time.num_seconds().max(0);
                let session_remaining_secs = (session_planned_secs - session_elapsed_secs).max(0);

                // Build the session status
                let mut session_status = SessionStatus {
                    kind: session_kind,
                    state: session_state,
                    planned_secs: session_planned_secs,
                    elapsed_secs: session_elapsed_secs,
                    remaining_secs: session_remaining_secs,
                };

                if matches!(session_status.state, SessionState::Running)
                // Complete the session if needed
                    && session_remaining_secs == 0
                {
                    let session_event = &SessionEvent::completed(session.id);
                    let params = InsertSessionEventArgs { session_event };
                    self.querier.insert_session_event(&params)?;
                    // Determine the session state from the last event
                    session_status.state = SessionState::from(&session_event.kind);

                    if let Some(runner) = &self.runner {
                        let args = SessionEventArgs {
                            session: session.clone(),
                            session_event: session_event.clone(),
                        };
                        // execute the hook
                        runner.execute(&args).ok();
                    }
                }

                self.render(&session_status, args)?;
            }
            None => {
                let status = SessionStatus::default();
                self.render(&status, args)?;
            }
        };

        Ok(())
    }

    /// Render `status` to stdout according to `args.output`.
    ///
    /// - `--output json`: pretty-printed JSON via `serde_json`.
    /// - `--output text`: MiniJinja template from `--format`, or [`DEFAULT_TEXT_TEMPLATE`].
    fn render(&self, status: &SessionStatus, args: &StatusCommandArgs) -> Result<()> {
        match args.output {
            StatusOutput::Json => {
                println!("{}", serde_json::to_string_pretty(status)?);
            }
            StatusOutput::Text => {
                let template = args.format.as_deref().unwrap_or(DEFAULT_TEXT_TEMPLATE);
                let output = Environment::new().render_str(template, status)?;
                println!("{}", output);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    /// Open an in-memory database, apply the schema, and return it.
    ///
    /// Used by every test in this module as the starting point for a clean,
    /// isolated database that is discarded when the test completes.
    fn setup() -> Result<Database> {
        let db = Database::open_in_memory()?;
        db.migrate()?;
        Ok(db)
    }

    /// Insert a session and the events returned by `f` into the DB.
    ///
    /// `f` receives the persisted [`Session`] so that event constructors can
    /// reference the correct `session_id`. Return one event per seed state
    /// transition needed by the test.
    fn seed_event<F>(db: &Database, f: F) -> Result<()>
    where
        F: Fn(&Session) -> Vec<SessionEvent>,
    {
        let querier = Querier::new(db.connection());
        let session = querier.insert_session(&InsertSessionArgs {
            session: &Session::default(),
        })?;
        for event in f(&session) {
            querier.insert_session_event(&InsertSessionEventArgs {
                session_event: &event,
            })?;
        }
        Ok(())
    }

    /// Fetch all session events and invoke `f(index, event)` for each one.
    ///
    /// Events are ordered by `created_at DESC`, so index `0` is always the most
    /// recent event. Use this to make per-event assertions without manually
    /// fetching or enumerating the list.
    fn for_each_event<F>(db: &Database, f: F) -> Result<()>
    where
        F: Fn(usize, &SessionEvent),
    {
        let querier = Querier::new(db.connection());
        let args = &ListSessionEventsArgs::default();
        let result = querier.list_session_events(args)?;
        for (index, event) in result.iter().enumerate() {
            f(index, event);
        }
        Ok(())
    }

    // --- StartCommand ---

    #[test]
    fn start_with_no_prior_events_starts_new_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        let cmd = StartCommand {
            runner: None,
            querier,
        };
        let args = &StartCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Started),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn start_when_session_is_started_does_nothing() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is currently running — start should be a no-op.
            vec![SessionEvent::started(session.id)]
        })?;

        let cmd = StartCommand {
            runner: None,
            querier,
        };
        let args = &StartCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Started),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn start_when_session_is_resumed_does_nothing() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session was resumed and is currently running — start should be a no-op.
            vec![SessionEvent::resumed(session.id)]
        })?;

        let cmd = StartCommand {
            runner: None,
            querier,
        };
        let args = &StartCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Resumed),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn start_when_session_is_paused_resumes_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is paused — start should resume it.
            vec![SessionEvent::paused(session.id)]
        })?;

        let cmd = StartCommand {
            runner: None,
            querier,
        };
        let args = &StartCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Resumed),
            1 => assert_eq!(event.kind, SessionEventKind::Paused),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn start_after_aborted_session_starts_new_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Previous session was aborted — start should begin a new one.
            vec![SessionEvent::aborted(session.id)]
        })?;

        let cmd = StartCommand {
            runner: None,
            querier,
        };
        let args = &StartCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Started),
            1 => assert_eq!(event.kind, SessionEventKind::Aborted),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn start_after_completed_session_starts_new_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Previous session completed naturally — start should begin a new one.
            vec![SessionEvent::completed(session.id)]
        })?;

        let cmd = StartCommand {
            runner: None,
            querier,
        };
        let args = &StartCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Started),
            1 => assert_eq!(event.kind, SessionEventKind::Completed),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    // --- StopCommand ---

    #[test]
    fn stop_with_no_prior_events_does_nothing() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, _event| {
            panic!("unexpected event at index {index}")
        })
    }

    #[test]
    fn stop_when_session_is_started_pauses_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is currently running — stop should pause it.
            vec![SessionEvent::started(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Paused),
            1 => assert_eq!(event.kind, SessionEventKind::Started),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn stop_when_session_is_started_with_reset_aborts_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is currently running — stop --reset should abort it.
            vec![SessionEvent::started(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs { reset: true };
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Aborted),
            1 => assert_eq!(event.kind, SessionEventKind::Started),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn stop_when_session_is_resumed_pauses_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session was resumed and is running — stop should pause it.
            vec![SessionEvent::resumed(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Paused),
            1 => assert_eq!(event.kind, SessionEventKind::Resumed),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn stop_when_session_is_resumed_with_reset_aborts_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session was resumed and is running — stop --reset should abort it.
            vec![SessionEvent::resumed(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs { reset: true };
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Aborted),
            1 => assert_eq!(event.kind, SessionEventKind::Resumed),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn stop_when_session_is_paused_does_nothing() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is paused — stop should be a no-op.
            vec![SessionEvent::paused(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Paused),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn stop_when_session_is_paused_with_reset_aborts_session() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is paused — stop --reset should abort it.
            vec![SessionEvent::paused(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs { reset: true };
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Aborted),
            1 => assert_eq!(event.kind, SessionEventKind::Paused),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn stop_when_session_is_aborted_does_nothing() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session was aborted — stop should be a no-op.
            vec![SessionEvent::aborted(session.id)]
        })?;

        let cmd = StopCommand {
            runner: None,
            querier,
        };
        let args = &StopCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Aborted),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    // --- StatusCommand ---

    #[test]
    fn status_when_session_is_already_completed_does_not_insert_another_completed() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session was already completed — status must not insert a second completed event.
            vec![
                SessionEvent::started(session.id),
                SessionEvent::completed(session.id),
            ]
        })?;

        let cmd = StatusCommand {
            runner: None,
            querier,
        };
        let args = &StatusCommandArgs::default();
        cmd.execute(args)?;

        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Completed),
            1 => assert_eq!(event.kind, SessionEventKind::Started),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn status_with_running_session_renders_text_output() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| {
            // Session is currently running — status should display it without auto-completing.
            vec![SessionEvent::started(session.id)]
        })?;

        let cmd = StatusCommand {
            runner: None,
            querier,
        };
        let args = &StatusCommandArgs::default();
        cmd.execute(args)?;

        // Session still has time remaining — no completed event should be inserted.
        for_each_event(&db, |index, event| match index {
            0 => assert_eq!(event.kind, SessionEventKind::Started),
            _ => panic!("unexpected event at index {index}"),
        })
    }

    #[test]
    fn status_with_running_session_renders_json_output() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| vec![SessionEvent::started(session.id)])?;

        let cmd = StatusCommand {
            runner: None,
            querier,
        };
        let args = &StatusCommandArgs {
            output: StatusOutput::Json,
            format: None,
        };
        cmd.execute(args)
    }

    #[test]
    fn status_with_running_session_renders_custom_text_format() -> Result<()> {
        let db = setup()?;
        let querier = Querier::new(db.connection());

        seed_event(&db, |session| vec![SessionEvent::started(session.id)])?;

        let cmd = StatusCommand {
            runner: None,
            querier,
        };
        let args = &StatusCommandArgs {
            output: StatusOutput::Text,
            format: Some("{{ remaining_secs }}s left".to_string()),
        };
        cmd.execute(args)
    }
}
