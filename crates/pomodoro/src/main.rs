mod app;
mod state;

use app::cli::*;
use app::cmd::*;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We create a new instance of the Database struct,
    // which will handle the connection to the SQLite.
    let database = state::schema::Database::new();
    // Migrate the database to ensure the necessary tables and schema are set up before executing
    // any commands.
    database.migrate().expect("Failed to migrate database");

    // Parse the command-line arguments into a Program struct, which contains the subcommand to
    // execute. The clap crate handles the parsing and validation of the command-line input based
    // on the defined structure in the cli module.
    let program = Program::parse();
    // Match the parsed command and execute the corresponding command logic.
    match program.command {
        ProgramCommand::Start(args) => {
            let command = StartCommand {};
            command.execute(args)
        }
        ProgramCommand::Stop(args) => {
            let command = StopCommand {};
            command.execute(args)
        }
        ProgramCommand::Status(args) => {
            let command = StatusCommand {};
            command.execute(args)
        }
    }
}
