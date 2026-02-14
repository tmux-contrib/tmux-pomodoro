use crate::app::cli::*;
use std::error::Error;
use std::time::Duration;

// StartCommand is responsible for starting a new pomodoro timer session.
pub struct StartCommand {}

impl StartCommand {
    /// Execute the StartCommand with the provided arguments.
    pub fn execute(&self, args: StartCommandArgs) -> Result<(), Box<dyn Error>> {
        let duration = args.duration.unwrap_or_else(|| match args.mode {
            StartMode::Work => Duration::from_secs(25 * 60),
            StartMode::Break => Duration::from_secs(5 * 60),
        });

        println!(
            "Starting a {:?} session for {} seconds.",
            args.mode,
            duration.as_secs()
        );

        Ok(())
    }
}

// StopCommand is responsible for stopping the current pomodoro timer session. It can also reset
// themand is responsible for stopping the current pomodoro timer session.
pub struct StopCommand {}

impl StopCommand {
    /// Execute the StopCommand with the provided arguments.
    pub fn execute(&self, args: StopCommandArgs) -> Result<(), Box<dyn Error>> {
        if args.reset {
            println!("Stopping and resetting the pomodoro timer.");
        } else {
            println!("Stopping the pomodoro timer.");
        }

        Ok(())
    }
}

/// StatusCommand is responsible for displaying the current status of the pomodoro timer, including
/// the remaining time, session type, and any relevant notifications. It can output the status in
/// different formats based on user preferences.
pub struct StatusCommand {}

impl StatusCommand {
    /// Execute the StatusCommand with the provided arguments.
    pub fn execute(&self, _args: StatusCommandArgs) -> Result<(), Box<dyn Error>> {
        println!("Displaying the current pomodoro timer status.");

        Ok(())
    }
}
