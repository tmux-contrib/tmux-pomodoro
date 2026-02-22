use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Deserialize;
use std::time::Duration;

/// Default MiniJinja template used by `--text` when no custom template string is provided.
pub const DEFAULT_TEXT_TEMPLATE: &str = "{{ kind }} | {{ state }} | elapsed {{ '%02d:%02d' | format(elapsed_secs // 60, elapsed_secs % 60) }} | remaining {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}";

/// Runtime configuration loaded from `$XDG_CONFIG_HOME/pomodoro/config.toml`.
///
/// All fields are optional in the file; missing keys fall back to the
/// [`Default`] values (25 min focus, 5 min break).
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ProgramConfig {
    /// Duration of a focus session (default: 25 minutes).
    #[serde(with = "humantime_serde")]
    pub focus_duration: Duration,
    /// Duration of a break session (default: 5 minutes).
    #[serde(with = "humantime_serde")]
    pub break_duration: Duration,
}

impl ProgramConfig {
    /// Load configuration from `$XDG_CONFIG_HOME/pomodoro/config.toml`.
    ///
    /// Returns an error if the file cannot be read or parsed. Callers
    /// should fall back to [`Default`] when the file does not exist.
    pub fn load() -> Result<Self> {
        let path = xdg::BaseDirectories::with_prefix("pomodoro")
            .place_config_file("config.toml")
            .context("Failed to determine configuration path")?;

        let content = std::fs::read(path).context("Failed to read configuration file")?;
        toml::from_slice(&content[..]).context("Failed to load configuration file")
    }
}

/// Returns the default configuration: 25-minute focus sessions and 5-minute break sessions.
impl Default for ProgramConfig {
    fn default() -> Self {
        Self {
            focus_duration: Duration::from_secs(25 * 60),
            break_duration: Duration::from_secs(5 * 60),
        }
    }
}

/// Program is the main entry point for the pomodoro timer CLI application.
#[derive(Parser)]
#[command(name = "pomodoro")]
#[command(about = "A simple pomodoro timer", version)]
pub struct Program {
    /// Use an ephemeral in-memory database (data is not persisted)
    #[arg(
        long = "in-memory",
        global = true,
        default_value_t = false,
        hide = true
    )]
    pub in_memory: bool,

    /// Skip hook execution for this invocation.
    #[arg(long = "no-hooks", global = true, default_value_t = false, hide = true)]
    pub no_hooks: bool,

    /// Command specifies the subcommand to execute.
    #[command(subcommand)]
    pub command: ProgramCommand,
}

/// Top-level subcommand dispatched by [`Program`].
#[derive(Debug, Subcommand)]
pub enum ProgramCommand {
    /// StartCommand is responsible for starting a new pomodoro timer session.
    #[command(name = "start")]
    #[command(about = "Start a new pomodoro timer session")]
    Start(StartCommandArgs),

    /// StopCommand is responsible for stopping the current pomodoro timer session.
    #[command(name = "stop")]
    #[command(about = "Stop the current pomodoro timer session")]
    Stop(StopCommandArgs),

    /// StatusCommand is responsible for displaying the current status of the pomodoro timer.
    #[command(name = "status")]
    #[command(about = "Display the current pomodoro timer status")]
    Status(StatusCommandArgs),
}

/// StartMode defines the session mode for the StartCommand.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum StartMode {
    /// Focus mode is the default session type for the pomodoro timer, where users focus on their
    /// tasks.
    #[default]
    Focus,

    /// Break mode is a session type for the pomodoro timer that allows users to take a short or
    /// long break.
    Break,
}

impl std::fmt::Display for StartMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Focus => write!(f, "focus"),
            Self::Break => write!(f, "break"),
        }
    }
}

/// StartCommandArgs defines the arguments for the StartCommand.
#[derive(Debug, Args, Default)]
pub struct StartCommandArgs {
    /// Mode specifies the type of session to start, either "focus" or "break". The default is
    /// "focus".
    #[arg(help = "The session mode")]
    #[arg(default_value_t = StartMode::Focus)]
    #[arg(short, long)]
    pub mode: StartMode,

    /// Duration specifies the length of the pomodoro timer session. The default is 25 minutes for
    /// focus sessions and 5 minutes for break sessions. The duration can be specified in a
    /// human-readable format (e.g., "25m" for 25 minutes, "1h" for 1 hour) and will be parsed
    /// using the humantime crate.
    #[arg(help = "The duration of the pomodoro timer")]
    #[arg(value_parser = humantime::parse_duration)]
    #[arg(short, long)]
    pub duration: Option<Duration>,
}

impl StartCommandArgs {
    /// Fill in `duration` from `config` when the user did not pass `--duration`.
    ///
    /// The config-sourced default depends on `mode`: focus sessions use
    /// `config.focus_duration`, break sessions use `config.break_duration`.
    pub fn with_config(mut self, config: &ProgramConfig) -> Self {
        if self.duration.is_none() {
            self.duration = Some(match self.mode {
                StartMode::Focus => config.focus_duration,
                StartMode::Break => config.break_duration,
            });
        }
        self
    }
}

/// Arguments for the `stop` subcommand.
#[derive(Debug, Args, Default)]
pub struct StopCommandArgs {
    /// Reset specifies whether to reset the pomodoro timer to zero when stopping.
    #[arg(help = "Reset the pomodoro timer to zero")]
    #[arg(short, long)]
    pub reset: bool,
}

/// StatusOutput defines the output format for the StatusCommand.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum StatusOutput {
    /// Text output is a human-readable format that displays the status of the pomodoro timer in a
    /// simple and concise way.
    #[default]
    Text,

    /// Json output is a machine-readable format that provides the status of the pomodoro timer in a
    /// structured way, making it easier to integrate with other tools or scripts.
    Json,
}

impl std::fmt::Display for StatusOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Json => write!(f, "json"),
        }
    }
}

/// StatusCommandArgs defines the arguments for the StatusCommand.
#[derive(Debug, Args, Default)]
pub struct StatusCommandArgs {
    /// Output specifies the format for displaying the status of the pomodoro timer.
    #[arg(help = "The output type")]
    #[arg(default_value_t = StatusOutput::Text)]
    #[arg(short, long)]
    pub output: StatusOutput,

    /// Format specifies a custom MiniJinja template for text output.
    #[arg(help = "Custom MiniJinja template for text output")]
    #[arg(short, long)]
    pub format: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_output_displays_as_text() {
        let output = StatusOutput::Text;
        assert_eq!(output.to_string(), "text");
    }

    #[test]
    fn json_output_displays_as_json() {
        let output = StatusOutput::Json;
        assert_eq!(output.to_string(), "json");
    }

    #[test]
    fn focus_mode_displays_as_focus() {
        let mode = StartMode::Focus;
        assert_eq!(mode.to_string(), "focus");
    }

    #[test]
    fn break_mode_displays_as_break() {
        let mode = StartMode::Break;
        assert_eq!(mode.to_string(), "break");
    }

    #[test]
    fn with_config_uses_break_duration_for_break_mode() {
        let config = ProgramConfig::default();
        let args = StartCommandArgs {
            mode: StartMode::Break,
            duration: None,
        };
        let result = args.with_config(&config);
        assert_eq!(result.duration, Some(config.break_duration));
    }

    #[test]
    fn with_config_preserves_provided_duration() {
        let config = ProgramConfig::default();
        let custom = std::time::Duration::from_secs(45 * 60);
        let args = StartCommandArgs {
            mode: StartMode::Focus,
            duration: Some(custom),
        };
        let result = args.with_config(&config);
        assert_eq!(result.duration, Some(custom));
    }
}
