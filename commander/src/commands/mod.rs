use std::process;

use crate::constants::VERSION;
use anyhow::Result;
use clap::{ArgMatches, Command};

pub mod demo;

// use std::process;

const ABOUT: &str = "
Command line utility for Perfect World Daemon.

This tools helps you manage yur server resources easy.
Use `--help` to learn more about this tool.
";

fn add_commands(mut app: Command) -> Command {
    let cmd = crate::commands::demo::make_command(Command::new("demo"));

    app = app.subcommand(cmd);

    app
}

fn run_command(matches: &ArgMatches) -> Result<()> {
    // let cmd = vec!["demo"];
    let cmd = "demo";

    if let Some(sub_matches) = matches.subcommand_matches("demo") {
        let rv = crate::commands::demo::execute(&sub_matches)?;
        return Ok(rv);
    }

    Ok(())
}

fn app() -> Command<'static> {
    Command::new("pwdaemon-cli")
        .version(VERSION)
        .about(ABOUT)
        .max_term_width(100)
}

fn execute() -> Result<()> {
    let mut app = app();
    app = add_commands(app);
    let matches = app.get_matches();
    run_command(&matches)
}

pub fn main() {
    let exit_code = match execute() {
        Ok(()) => 0,
        Err(err) => {
            if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
                clap_err.exit();
            }

            eprintln!("{} {}", "Error:", err);
            err.chain()
                .skip(1)
                .for_each(|cause| eprintln!("  {} {}", "Caused by", cause.source().unwrap()));
            1
        }
    };

    process::exit(exit_code)
}
