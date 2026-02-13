use std::fmt::Display;
use std::time::Duration;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Program is the main entry point for the pomodoro timer CLI application.
#[derive(Parser)]
#[command(name = "pomodoro")]
#[command(about = "A simple pomodoro timer")]
pub struct Program {
    /// Command specifies the subcommand to execute.
    #[command(subcommand)]
    pub command: ProgramCommand,
}

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
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum StartMode {
    /// Work mode is the default session type for the pomodoro timer, where users focus on their
    /// tasks.
    Work,

    /// Break mode is a session type for the pomodoro timer that allows users to take a short or
    /// long break.
    Break,
}

impl Display for StartMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartMode::Work => write!(f, "work"),
            StartMode::Break => write!(f, "break"),
        }
    }
}

/// StartCommandArgs defines the arguments for the StartCommand.
#[derive(Debug, Args)]
pub struct StartCommandArgs {
    /// Mode specifies the type of session to start, either "work" or "break". The default is
    /// "work".
    #[arg(help = "The session mode")]
    #[arg(default_value_t = StartMode::Work)]
    #[arg(short, long)]
    pub mode: StartMode,

    /// Duration specifies the length of the pomodoro timer session. The default is 25 minutes for
    /// work sessions and 5 minutes for break sessions. The duration can be specified in a
    /// human-readable format (e.g., "25m" for 25 minutes, "1h" for 1 hour) and will be parsed
    /// using the humantime crate.
    #[arg(help = "The duration of the pomodoro timer")]
    #[arg(value_parser = humantime::parse_duration)]
    #[arg(default_value_if("mode", "work", "25m"))]
    #[arg(default_value_if("mode", "break", "5m"))]
    #[arg(short, long)]
    pub duration: Option<Duration>,
}

#[derive(Debug, Args)]
pub struct StopCommandArgs {
    /// Reset specifies whether to reset the pomodoro timer to zero when stopping.
    #[arg(help = "Reset the pomodoro timer to zero")]
    #[arg(short, long)]
    pub reset: bool,
}

/// StatusOutput defines the output format for the StatusCommand.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum StatusOutput {
    // Text output is a human-readable format that displays the status of the pomodoro timer in a
    // simple and concise way.
    Text,

    // Json output is a machine-readable format that provides the status of the pomodoro timer in a
    // structured way, making it easier to integrate with other tools or scripts.
    Json,
}

impl Display for StatusOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusOutput::Text => write!(f, "text"),
            StatusOutput::Json => write!(f, "json"),
        }
    }
}

/// StatusCommandArgs defines the arguments for the StatusCommand.
#[derive(Debug, Args)]
pub struct StatusCommandArgs {
    /// Output specifies the format for displaying the status of the pomodoro timer.
    #[arg(help = "The output type")]
    #[arg(default_value_t = StatusOutput::Text)]
    #[arg(short, long)]
    pub output: StatusOutput,

    // Format specifies a custom format string for the status output.
    #[arg(help = "Custom format string for the status output")]
    #[arg(short, long)]
    pub format: Option<String>,
}
