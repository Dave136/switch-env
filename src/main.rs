mod git;
mod log;
mod workspace;

use clap::{arg, Command};
use log::Log;
use std::io::Result as IoResult;
use workspace::{Env, Workspace};

fn main() -> IoResult<()> {
    let program = Command::new("switch-env")
        .author("Dave136")
        .version("0.0.2")
        .about("It allows us to change ssh configuration and global git configuration between personal or work environment")
        .arg(arg!(-e --env <ENV> "Set the workspace environment").required(false))
        .arg(arg!(-s --status ... "Shows the current workspace environment"))
        .get_matches();

    if program.is_present("env") {
        let mut workspace = Workspace::new();

        match program.value_of("env") {
            Some(value) => match value {
                "personal" => workspace.create_config(Env::Personal)?,
                "work" => workspace.create_config(Env::Work)?,
                _ => Log::error("Not valid workspace"),
            },
            None => Log::error("Please, provide an valid option"),
        };

        return Ok(());
    }

    if program.is_present("status") {
        let mut workspace = Workspace::new();

        match program.occurrences_of("status") {
            0 => Log::info("No showing status"),
            1 => workspace.show_status()?,
            _ => Log::error("No more occurrences allowed!"),
        };
    }

    Ok(())
}
