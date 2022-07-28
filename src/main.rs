mod environment;
mod git;
mod log;
mod workspace;

use clap::{arg, Command};
use environment::Environment;
use log::Log;
use std::io;
use workspace::Workspace;

fn main() -> io::Result<()> {
    let environment = Environment::new();

    let program = Command::new("switch-env")
        .author("Dave136")
        .version("0.2.0")
        .about("It allows us to change the global git configuration between environments")
        .arg(arg!(-e --env <ENV> "Set the workspace environment").required(false))
        .arg(arg!(-s --status ... "Shows the current workspace environment"))
        .get_matches();

    if environment.list.len() == 0 {
        Log::success("Not available workspace environment");
        Log::info("You can extend the workspace environment updating the file located in \"$HOME/.switch-env/env.json\"");
        return Ok(());
    }

    if program.is_present("env") {
        let mut workspace = Workspace::new();

        match program.value_of("env") {
            Some(value) => {
                let result = environment.list.iter().find(|env| env.name == value);

                match result {
                    Some(env) => {
                        workspace.create_config(&env.credentials)?;
                    }
                    None => Log::error("Workspace not found"),
                }
            }
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
