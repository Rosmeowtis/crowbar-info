use clap::{Arg, Command};

pub enum Cmd {
    Info,
    Players,
    Full,
}

pub fn cli() -> (Cmd, Vec<String>) {
    let matches = arg_parser().get_matches();
    match matches.subcommand() {
        Some(("info", subm)) => (
            Cmd::Info,
            subm.get_many::<String>("host")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
        ),
        Some(("players", subm)) => (
            Cmd::Players,
            subm.get_many::<String>("host")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
        ),
        Some(("full", subm)) => (
            Cmd::Full,
            subm.get_many::<String>("host")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
        ),
        _ => panic!("No subcommand was used"),
    }
}

pub fn arg_parser() -> Command {
    let arg_hosts = Arg::new("host")
        .help("server addresses, ip:port OR domain:port")
        .required(true)
        .num_args(1..);
    let cmd = Command::new("cbi")
        .version("0.1.0")
        .author("Rosmeowtis <rosmeowtis@outlook.com>")
        .about("query server info or players list of srcds servers")
        .subcommand(
            Command::new("info")
                .about("a2s_info")
                .arg(arg_hosts.clone()),
        )
        .subcommand(
            Command::new("players")
                .about("a2s_players")
                .arg(arg_hosts.clone()),
        )
        .subcommand(
            Command::new("full")
                .about("merge a2s_info and a2s_players")
                .arg(arg_hosts.clone()),
        );

    cmd
}
