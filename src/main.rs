mod account;
mod bot;
mod rules;
mod schema;

use clap::Command;
use diesel::{Connection, SqliteConnection};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let m = Command::new("bot")
        .about("Bot to manage events on the fio API")
        .subcommands([
            Command::new("account").subcommands(account::subcommands::generate_subcommands()),
            Command::new("rule").subcommands(rules::subcommands::generate_subcommands()),
        ])
        .get_matches();
    match m.subcommand() {
        None => {
            bot::check_accounts().await;
        }
        Some(command) => match command.0 {
            "account" => match command.1.subcommand() {
                None => {
                    error!("You must specify a subcommand");
                    info!("add: Add an account to the bot");
                    info!("remove: Remove an account from the bot");
                    info!("list: List all accounts");
                }
                Some(subcommand) => {
                    match subcommand.0 {
                        "add" => {
                            let name = subcommand.1.get_one::<String>("name").unwrap();
                            let number = subcommand.1.get_one::<i32>("number").unwrap();
                            let token = subcommand.1.get_one::<String>("token").unwrap();
                            let read_only = subcommand.1.get_flag("read_only");
                            account::add(&name, number, &token, &read_only);
                        }
                        "remove" => {
                            let name = subcommand.1.get_one::<String>("name").unwrap();
                            account::remove(&name);
                        }
                        "list" => {
                            let show_tokens = subcommand.1.get_flag("show_tokens");
                            account::list(show_tokens);
                        }
                        _ => {}
                    };
                }
            },
            "rule" => match command.1.subcommand() {
                None => {
                    error!("You must specify a subcommand");
                    info!("add: Add a rule to the bot");
                    info!("remove: Remove a rule from the bot");
                    info!("list: List all rules");
                }
                Some(subcommand) => {
                    match subcommand.0 {
                        "add" => {
                            let target_account =
                                subcommand.1.get_one::<String>("target_account").unwrap();
                            let account = subcommand.1.get_one::<i32>("account").unwrap();
                            let amount = subcommand.1.get_one::<i32>("amount").unwrap();
                            let target_bank = subcommand.1.get_one::<str>("target_bank");
                            let bic = subcommand.1.get_one::<str>("bic");
                            let ks = subcommand.1.get_one::<i32>("ks");
                            let vs = subcommand.1.get_one::<i32>("vs");
                            let ss = subcommand.1.get_one::<i32>("ss");
                            let message = subcommand.1.get_one::<str>("message");
                            let comment = subcommand.1.get_one::<str>("comment");
                            let for_ = subcommand.1.get_one::<str>("for");
                            let payment_type = subcommand
                                .1
                                .get_one::<i32>("payment_type")
                                .unwrap_or(&431001);
                            let active = subcommand.1.get_flag("active");
                            let percent = subcommand.1.get_flag("percent");
                            let sequence = subcommand.1.get_one::<i32>("sequence");
                            rules::add(
                                account,
                                amount,
                                target_account,
                                target_bank,
                                bic,
                                ks,
                                vs,
                                ss,
                                message,
                                comment,
                                for_,
                                payment_type,
                                active,
                                percent,
                                sequence,
                            );
                        }
                        "remove" => {
                            let name = subcommand.1.get_one::<String>("name").unwrap();
                            // rules::remove(&name);
                        }
                        "list" => {
                            // rules::list();
                        }
                        _ => {}
                    };
                }
            },
            _ => {}
        },
    }
}

pub(crate) fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
