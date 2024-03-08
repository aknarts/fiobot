use clap::{Arg, Command};

pub fn generate() -> Vec<Command> {
    vec![
        Command::new("add")
            .about("Add a rule to the bot")
            .arg(
                Arg::new("account")
                    .short('a')
                    .long("account")
                    .help("Account the email is for")
                    .value_parser(clap::value_parser!(i64))
                    .required(false),
            )
            .arg(
                Arg::new("mail")
                    .short('m')
                    .long("mail")
                    .help("Email to send notification to")
                    .required(true),
            )
            .arg(
                Arg::new("default")
                    .short('d')
                    .action(clap::ArgAction::SetTrue)
                    .long("default")
                    .help("Is default email (account gets ignored)")
                    .required(false),
            ),
        Command::new("remove")
            .about("Remove a email from the bot")
            .arg(
                Arg::new("account")
                    .short('a')
                    .long("account")
                    .help("Account the email is for")
                    .value_parser(clap::value_parser!(i64))
                    .required(false),
            ),
        Command::new("list").about("List all emails"),
    ]
}
