use clap::{crate_version, Command};

fn main() {
    let matches = Command::new("school42")
        .version(crate_version!())
        .about("Some commands are used for the school42")
        .subcommand_required(true)
        .get_matches();

    match matches.subcommand() {
        _ => unreachable!(),
    }
}
