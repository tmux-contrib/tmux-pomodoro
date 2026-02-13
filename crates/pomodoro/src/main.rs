mod app;

use app::cli::*;
use app::cmd::*;
use clap::Parser;

fn main() {
    let program = Program::parse();

    match program.command {
        ProgramCommand::Start(args) => {
            let command = StartCommand {};
            command.execute(args);
        }
        ProgramCommand::Stop(args) => {
            let command = StopCommand {};
            command.execute(args);
        }
        ProgramCommand::Status(args) => {
            let command = StatusCommand {};
            command.execute(args);
        }
    }
}
