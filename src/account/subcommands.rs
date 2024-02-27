use clap::{Arg, Command};

pub fn generate() -> Vec<Command> {
    vec![
        Command::new("add")
            .about("Add an account to the bot")
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .help("Name of the account to add")
                    .required(true),
            )
            .arg(
                Arg::new("number")
                    .short('b')
                    .long("number")
                    .value_parser(clap::value_parser!(i32))
                    .help("Number of the account to add")
                    .required(true),
            )
            .arg(
                Arg::new("token")
                    .short('t')
                    .long("token")
                    .help("Token of the account to add")
                    .required(true),
            )
            .arg(
                Arg::new("read_only")
                    .short('r')
                    .action(clap::ArgAction::SetTrue)
                    .long("read_only")
                    .help("Token of the account to add")
                    .required(false),
            ),
        Command::new("remove")
            .about("Remove an account from the bot")
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .help("Name of the account to remove")
                    .required(true),
            ),
        Command::new("list").about("List all accounts").arg(
            Arg::new("show_tokens")
                .short('t')
                .action(clap::ArgAction::SetTrue)
                .long("show_tokens")
                .help("Show tokens of the accounts")
                .required(false),
        ),
    ]
}
