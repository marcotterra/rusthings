use anyhow::Result;
use clap::{ArgMatches, Command};

pub mod todo;

pub fn make_command(mut command: Command) -> Command {
    command = command //
        .subcommand(crate::commands::demo::todo::make_command(Command::new(
            "todo",
        )));

    command = command
        .about("Demonstation")
        .subcommand_required(true)
        .arg_required_else_help(true);

    command
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    if let Some(sub_matches) = matches.subcommand_matches("todo") {
        return crate::commands::demo::todo::execute(sub_matches);
    }

    unreachable!()
}
