use clap::{ArgMatches, Command};

mod compile;

pub fn command() -> Command<'static> {
    Command::new("c")
        .about("Some useful c commands")
        .subcommand_required(true)
        .subcommand(compile::command())
}

pub fn execute(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("compile", sub_matches)) => compile::execute(sub_matches),
        _ => unreachable!(),
    }
}
