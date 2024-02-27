use crate::schema::accounts;
use diesel::query_dsl::methods::SelectDsl;
use diesel::RunQueryDsl;
use fiocz_rs::types::account_statement::TransactionDataEnum;
use tracing::{error, info};

pub async fn check_accounts() {
    let mut conn = crate::establish_connection();
    let result = accounts::table
        .select(accounts::all_columns)
        .load::<crate::account::Account>(&mut conn);
    match result {
        Ok(accounts) => {
            for account in accounts {
                let token = crate::account::get_token(&account.name, false);
                if token.is_none() {
                    error!("Account {} has no token", account.name);
                }
                let fio = fiocz_rs::Fio::new(&token.unwrap());
                match fio.set_last_id("26329634966").await {
                    Ok(()) => {
                        info!("Set new stop");
                    }
                    Err(e) => {
                        error!("Failed to set last id: {:?}", e);
                    }
                };

                let mut sum = rust_decimal::Decimal::new(0, 0);

                match fio.movements_since_last().await {
                    Ok(statement) => {
                        for transaction in statement.account_statement.transaction_list.transaction
                        {
                            for (key, value) in transaction {
                                if key.to_ascii_lowercase() != "column1" {
                                    continue;
                                }
                                if value.is_none() {
                                } else if let Some(data) = value {
                                    if let TransactionDataEnum::Decimal(f) = data.value {
                                        sum += f;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to get newest account movements: {:?}", e);
                    }
                }

                info!("Account {} has sum {}", account.name, sum);
            }
        }
        Err(e) => error!("Could not get accounts: {}", e),
    }
}
