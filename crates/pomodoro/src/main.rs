mod app;
mod hook;
mod state;

use crate::app::cli::*;
use crate::app::cmd::*;
use crate::hook::run::*;
use crate::state::query::*;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::parse();
    let program_config = ProgramConfig::load().unwrap_or_default();

    // Create the hook runner unless --no-hooks was passed.
    let runner = if program.no_hooks {
        None
    } else {
        Some(Runner::try_new()?)
    };

    // Open (or create) the database. --in-memory uses an ephemeral SQLite
    // database that vanishes when the process exits; useful for testing and
    // one-shot runs where persistence is not required.
    let mut database = if program.in_memory {
        Database::open_in_memory()?
    } else {
        Database::open()?
    };
    // Migrate the datbase prior to its usage.
    database.migrate()?;

    // Wrap the entire command in a single transaction so that any partial
    // failure (e.g. session inserted but event write fails) rolls back cleanly.
    let tx = database.transaction()?;
    let querier = Querier::new(&tx);

    match program.command {
        ProgramCommand::Start(args) => {
            let args = args.with_config(&program_config);
            let command = StartCommand { runner, querier };
            command.execute(&args)?
        }
        ProgramCommand::Stop(args) => {
            let command = StopCommand { runner, querier };
            command.execute(&args)?
        }
        ProgramCommand::Status(args) => {
            let command = StatusCommand { runner, querier };
            command.execute(&args)?
        }
    }

    tx.commit()?;
    // We are done!
    Ok(())
}
