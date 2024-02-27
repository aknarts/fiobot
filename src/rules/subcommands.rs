use clap::{Arg, Command};

pub fn generate() -> Vec<Command> {
    vec![
        Command::new("add")
            .about("Add a rule to the bot")
            .arg(
                Arg::new("account")
                    .short('a')
                    .long("account")
                    .help("Account the rule is for")
                    .value_parser(clap::value_parser!(i32))
                    .required(true),
            )
            .arg(
                Arg::new("amount")
                    .short('m')
                    .long("amount")
                    .value_parser(clap::value_parser!(i32))
                    .help("How much to send")
                    .required(true),
            )
            .arg(
                Arg::new("target_account")
                    .short('t')
                    .long("target_account")
                    .help("Account to send the money to")
                    .required(true),
            )
            .arg(
                Arg::new("target_bank")
                    .short('u')
                    .long("target_bank")
                    .help("Account bank to send the money to")
                    .required(false),
            )
            .arg(
                Arg::new("bic")
                    .short('b')
                    .long("bic")
                    .help("BIC of the target bank")
                    .required(false),
            )
            .arg(
                Arg::new("ks")
                    .short('k')
                    .long("ks")
                    .value_parser(clap::value_parser!(i32))
                    .help("KS of the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("vs")
                    .short('v')
                    .long("vs")
                    .value_parser(clap::value_parser!(i32))
                    .help("VS of the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("ss")
                    .short('s')
                    .long("ss")
                    .value_parser(clap::value_parser!(i32))
                    .help("SS of the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("message")
                    .short('e')
                    .long("message")
                    .help("Message for the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("comment")
                    .short('c')
                    .long("comment")
                    .help("Comment for the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("for")
                    .short('f')
                    .long("for")
                    .help("Who is the transfer for")
                    .required(false),
            )
            .arg(
                Arg::new("type")
                    .short('w')
                    .long("type")
                    .value_parser(clap::value_parser!(i32))
                    .help("Type of transfer")
                    .required(false),
            )
            .arg(
                Arg::new("order")
                    .short('o')
                    .long("order")
                    .value_parser(clap::value_parser!(i32))
                    .help("In what order to execute the rule")
                    .required(false),
            )
            .arg(
                Arg::new("active")
                    .short('n')
                    .action(clap::ArgAction::SetTrue)
                    .long("active")
                    .help("Is the rule active")
                    .required(false),
            )
            .arg(
                Arg::new("percent")
                    .short('p')
                    .action(clap::ArgAction::SetTrue)
                    .long("percent")
                    .help("Is percent rule")
                    .required(false),
            ),
        Command::new("edit")
            .about("Edit a rule for the bot")
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .value_parser(clap::value_parser!(i32))
                    .help("Which rule to edit")
                    .required(true),
            )
            .arg(
                Arg::new("account")
                    .short('a')
                    .long("account")
                    .help("Account the rule is for")
                    .required(false),
            )
            .arg(
                Arg::new("amount")
                    .short('m')
                    .long("amount")
                    .value_parser(clap::value_parser!(i32))
                    .help("How much to send")
                    .required(false),
            )
            .arg(
                Arg::new("target_account")
                    .short('t')
                    .long("target_account")
                    .help("Account to send the money to")
                    .required(false),
            )
            .arg(
                Arg::new("target_bank")
                    .short('u')
                    .long("target_bank")
                    .value_parser(clap::value_parser!(i32))
                    .help("Account bank to send the money to")
                    .required(false),
            )
            .arg(
                Arg::new("bic")
                    .short('b')
                    .long("bic")
                    .help("BIC of the target bank")
                    .required(false),
            )
            .arg(
                Arg::new("ks")
                    .short('k')
                    .long("ks")
                    .value_parser(clap::value_parser!(i32))
                    .help("KS of the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("vs")
                    .short('v')
                    .long("vs")
                    .value_parser(clap::value_parser!(i32))
                    .help("VS of the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("ss")
                    .short('s')
                    .long("ss")
                    .value_parser(clap::value_parser!(i32))
                    .help("SS of the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("message")
                    .short('e')
                    .long("message")
                    .help("Message for the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("comment")
                    .short('c')
                    .long("comment")
                    .help("Comment for the transfer")
                    .required(false),
            )
            .arg(
                Arg::new("for")
                    .short('f')
                    .long("for")
                    .help("Who is the transfer for")
                    .required(false),
            )
            .arg(
                Arg::new("type")
                    .short('w')
                    .long("type")
                    .value_parser(clap::value_parser!(i32))
                    .help("Type of transfer")
                    .required(false),
            )
            .arg(
                Arg::new("order")
                    .short('o')
                    .long("order")
                    .value_parser(clap::value_parser!(i32))
                    .help("In what order to execute the rule")
                    .required(false),
            )
            .arg(
                Arg::new("active")
                    .short('n')
                    .action(clap::ArgAction::SetTrue)
                    .long("active")
                    .help("Is the rule active")
                    .required(false),
            )
            .arg(
                Arg::new("percent")
                    .short('p')
                    .action(clap::ArgAction::SetTrue)
                    .long("percent")
                    .help("Is percent rule")
                    .required(false),
            ),
        Command::new("remove")
            .about("Remove a rule from the bot")
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .help("ID of the rule to remove")
                    .required(true),
            ),
        Command::new("list").about("List all rules").arg(
            Arg::new("account")
                .short('a')
                .action(clap::ArgAction::SetTrue)
                .long("account")
                .help("Show rules for account accounts")
                .required(false),
        ),
    ]
}
