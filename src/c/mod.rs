use clap::{ArgMatches, Command};

use docker_images::utils::proxy_args;

pub fn command() -> Command<'static> {
    Command::new("c")
        .about("Some useful c commands")
        .arg(proxy_args::set_proxy_args(true))
}

pub fn execute(matches: &ArgMatches) {
    println!("{:?}", proxy_args::get_values_from_proxy_args(matches));
}
