use clap::{ArgMatches, Command};

use docker_images::utils::proxy_args;

pub fn command() -> Command<'static> {
    Command::new("compile")
        .about("Compile the c project in the school42")
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches) {
    println!("{:?}", proxy_args::get_values_from_proxy_args(matches));
}
