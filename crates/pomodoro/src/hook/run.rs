use crate::state::model::*;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Arguments passed to a hook script as a JSON payload over stdin.
///
/// Both fields are serialized together so the hook receives full context
/// about the session and the event that triggered it.
#[derive(Serialize, Deserialize)]
pub struct SessionEventArgs {
    /// The session associated with the event.
    pub session: Session,
    /// The event that triggered the hook.
    pub session_event: SessionEvent,
}

/// Executes user-defined hook scripts when session state changes.
///
/// Hook scripts live under `$XDG_CONFIG_HOME/pomodoro/hooks/` and are named
/// after the event kind: `start` for [`SessionEventKind::Started`] /
/// [`SessionEventKind::Resumed`], and `stop` for all other events.
/// A missing hook file is silently ignored.
pub struct Runner {
    /// Absolute path to the hooks directory (`…/pomodoro/hooks/`).
    path: PathBuf,
}

impl Runner {
    /// Build a [`Runner`] whose hooks directory is resolved from the XDG
    /// config home (typically `~/.config/pomodoro/hooks/`).
    ///
    /// Returns an error only if the XDG base-directory lookup itself fails.
    pub fn try_new() -> Result<Self> {
        let path = xdg::BaseDirectories::with_prefix("pomodoro")
            .get_config_home()
            .context("Failed to determine configuration path")?
            .join("hooks");

        Ok(Self { path })
    }

    /// Run the hook script that corresponds to the event in `args`.
    ///
    /// The script path is `<hooks_dir>/<name>` where `<name>` is `"start"` or
    /// `"stop"` (see [`Runner::name`]). If no file exists at that path the
    /// method returns `Ok(())` immediately.
    ///
    /// When the script exists it is spawned as a child process with its stdin
    /// connected to a pipe and stdout suppressed. A JSON-serialized
    /// [`SessionEventArgs`] is written to that pipe and the child is then
    /// detached — the method returns without waiting for the script to finish.
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization or process spawning fails.
    /// Call sites that treat hooks as non-fatal should discard the error
    /// with `.ok()`.
    pub fn execute(&self, args: &SessionEventArgs) -> Result<()> {
        let path = self.path.join(self.name(args));
        if !path.exists() {
            return Ok(());
        }

        let data = serde_json::to_string(args).context("Failed to serialize hook arguments")?;
        let mut process = Command::new(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .context("Failed to spawn hook")?;

        if let Some(mut stdin) = process.stdin.take() {
            stdin
                .write_all(data.as_bytes())
                .context("Failed to write hook arguments")?;
        }
        // Drop `process` without wait() — child runs detached; stdin EOF was already sent.
        Ok(())
    }

    /// Map an event to the hook file name: `"start"` for started/resumed
    /// events, `"stop"` for everything else.
    fn name(&self, args: &SessionEventArgs) -> &str {
        let kind = &args.session_event.kind;
        if matches!(kind, SessionEventKind::Started | SessionEventKind::Resumed) {
            "start"
        } else {
            "stop"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use uuid::Uuid;

    /// Create a [`Runner`] backed by a unique temporary hooks directory.
    fn setup() -> Result<Runner> {
        let path = std::env::temp_dir().join(format!("pomodoro-hook-{}", Uuid::now_v7()));
        fs::create_dir_all(&path)?;
        Ok(Runner { path })
    }

    /// Poll until `path` exists **and** has non-zero size, or a 500 ms deadline is reached.
    ///
    /// Checking size in addition to existence avoids a race where the shell has
    /// already created (opened) the output file but `cat` has not yet finished
    /// writing the hook payload to it.
    fn wait_for_file(path: &std::path::Path) -> bool {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
        while std::time::Instant::now() < deadline {
            if path.metadata().map(|m| m.len() > 0).unwrap_or(false) {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        path.metadata().map(|m| m.len() > 0).unwrap_or(false)
    }

    /// Install an executable hook script named `name` that captures its stdin
    /// JSON payload into `<hooks_dir>/<name>.json`. Returns the path to that
    /// output file so callers can assert on its existence and contents.
    fn install_hook(runner: &Runner, name: &str) -> Result<PathBuf> {
        let script = runner.path.join(name);
        let output = runner.path.join(format!("{name}.json"));
        fs::write(&script, format!("#!/bin/sh\ncat > {}", output.display()))?;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))?;
        Ok(output)
    }

    // --- missing hook ---

    #[test]
    fn missing_start_hook_returns_ok() -> Result<()> {
        let runner = setup()?;
        let session = Session::default();
        let session_event = SessionEvent::started(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)
    }

    #[test]
    fn missing_stop_hook_returns_ok() -> Result<()> {
        let runner = setup()?;
        let session = Session::default();
        let session_event = SessionEvent::paused(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)
    }

    // --- hook routing ---

    #[test]
    fn started_event_invokes_start_hook() -> Result<()> {
        let runner = setup()?;
        let path = install_hook(&runner, "start")?;

        let session = Session::default();
        let session_event = SessionEvent::started(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)?;

        assert!(
            wait_for_file(&path),
            "start hook was not invoked for started event"
        );
        Ok(())
    }

    #[test]
    fn resumed_event_invokes_start_hook() -> Result<()> {
        let runner = setup()?;
        let path = install_hook(&runner, "start")?;

        let session = Session::default();
        let session_event = SessionEvent::resumed(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)?;

        assert!(
            wait_for_file(&path),
            "start hook was not invoked for resumed event"
        );
        Ok(())
    }

    #[test]
    fn paused_event_invokes_stop_hook() -> Result<()> {
        let runner = setup()?;
        let path = install_hook(&runner, "stop")?;

        let session = Session::default();
        let session_event = SessionEvent::paused(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)?;

        assert!(wait_for_file(&path), "stop hook was not invoked for paused event");
        Ok(())
    }

    #[test]
    fn aborted_event_invokes_stop_hook() -> Result<()> {
        let runner = setup()?;
        let path = install_hook(&runner, "stop")?;

        let session = Session::default();
        let session_event = SessionEvent::aborted(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)?;

        assert!(wait_for_file(&path), "stop hook was not invoked for aborted event");
        Ok(())
    }

    #[test]
    fn completed_event_invokes_stop_hook() -> Result<()> {
        let runner = setup()?;
        let path = install_hook(&runner, "stop")?;

        let session = Session::default();
        let session_event = SessionEvent::completed(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)?;

        assert!(
            wait_for_file(&path),
            "stop hook was not invoked for completed event"
        );
        Ok(())
    }

    // --- JSON payload ---

    #[test]
    fn execute_writes_json_payload_to_hook_stdin() -> Result<()> {
        let runner = setup()?;
        let path = install_hook(&runner, "start")?;

        let session = Session::default();
        let session_event = SessionEvent::started(session.id);
        let args = SessionEventArgs {
            session: session.clone(),
            session_event: session_event.clone(),
        };
        runner.execute(&args)?;
        wait_for_file(&path);

        let content = fs::read_to_string(&path).unwrap();
        let output: SessionEventArgs = serde_json::from_str(&content).unwrap();

        assert_eq!(output.session.id, session.id);
        assert_eq!(output.session.kind, SessionKind::Focus);
        assert_eq!(output.session.planned_duration, session.planned_duration);
        assert_eq!(output.session_event.kind, SessionEventKind::Started);
        assert_eq!(output.session_event.session_id, session.id);
        Ok(())
    }
}
