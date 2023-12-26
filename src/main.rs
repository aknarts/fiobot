mod account;
mod schema;

use clap::{Arg, Command};
use diesel::{Connection, SqliteConnection};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let m = Command::new("bot")
        .about("Bot to manage events on the fio API")
        .subcommands([
            Command::new("account").subcommands(account::subcommands::generate_subcommands())
        ])
        .get_matches();
    match m.subcommand() {
        None => {}
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
            _ => {}
        },
    }
}

pub(crate) fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
