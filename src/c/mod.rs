use clap::{ArgMatches, Command};

pub fn command() -> Command<'static> {
    Command::new("c")
        .subcommand_required(true)
        .about("Some useful c commands")
}

pub fn execute(matches: &ArgMatches) {
    println!("{:?}", matches);
}
