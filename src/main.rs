use clap::{crate_version, Command};

mod c;

fn main() {
    let matches = Command::new("school42")
        .version(crate_version!())
        .about("Some commands are used for the school42")
        .subcommand_required(true)
        .subcommand(c::command())
        .get_matches();

    match matches.subcommand() {
        Some(("c", sub_matches)) => c::execute(sub_matches),
        _ => unreachable!(),
    }
}
