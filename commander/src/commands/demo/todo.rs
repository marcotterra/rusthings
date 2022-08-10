use anyhow::Result;
use clap::{ArgMatches, Command};

pub fn make_command(command: Command) -> Command {
    command
        .about("The initial command for demo")
        .allow_hyphen_values(true)
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    println!("TODO EXECUTED");
    Ok(())
}
