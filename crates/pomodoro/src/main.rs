mod cli;

use clap::Parser;
use cli::Program;

fn main() {
    let program = Program::parse();

    match program.command {
        cli::ProgramCommand::Start(args) => {
            let command = cli::StartCommand {};
            command.execute(args);
        }
        cli::ProgramCommand::Stop(args) => {
            let command = cli::StopCommand {};
            command.execute(args);
        }
        cli::ProgramCommand::Status(args) => {
            let command = cli::StatusCommand {};
            command.execute(args);
        }
    }
}
