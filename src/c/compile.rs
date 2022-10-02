use clap::{ArgMatches, Command};

use docker_images::utils::{proxy_args, docker, sub_process};

pub fn command() -> Command<'static> {
    Command::new("compile")
        .about("Compile the c project in the school42")
        .arg(proxy_args::set_proxy_args(false))
}

pub fn execute(matches: &ArgMatches) {
    docker::run(
        [
            vec![
                "-it",
                "--rm",
                "ghcr.io/hsuting/norminette:main",
                "norminette",
            ],
            proxy_args::get_values_from_proxy_args(matches),
        ]
            .concat(),
    );
    sub_process::exec(
        "gcc",
        [
            vec![
                "-Wall",
                "-Wextra",
                "-Werror",
            ],
            proxy_args::get_values_from_proxy_args(matches),
        ]
            .concat(),
    );
}
