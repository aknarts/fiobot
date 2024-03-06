use crate::account::get_accounts;
use fiocz_rs::types::account_statement::TransactionDataEnum;
use fiocz_rs::types::transaction::{ImportBuilder, Type};
use rust_decimal::prelude::FromPrimitive;
use tracing::{error, info};

pub async fn check_accounts() {
    let accounts = get_accounts();
    for account in accounts {
        let token = crate::account::get_token(&account.name, false);
        if token.is_none() {
            error!("Account {} has no token", account.name);
        }
        let fio = fiocz_rs::Fio::new(&token.unwrap());

        let mut sum = rust_decimal::Decimal::new(0, 0);

        match fio.movements_since_last().await {
            Ok(statement) => {
                let info = statement.account_statement.info;
                for transaction in statement.account_statement.transaction_list.transaction {
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
                info!("Account info: {:?}", info.clone());
                info!("Account {} has sum {}", account.name, sum);
                if sum > rust_decimal::Decimal::new(0, 0) {
                    let mut builder = ImportBuilder::new();
                    let rules = crate::rules::get_rules_for_account(&account.number);
                    for rule in &rules {
                        if rule.bic.is_none() {
                            let amount = if rule.percent > 0 {
                                (sum * rust_decimal::Decimal::from_f32(rule.amount / 100.0)
                                    .unwrap())
                                .round_dp_with_strategy(
                                    2,
                                    rust_decimal::RoundingStrategy::ToNegativeInfinity,
                                )
                            } else {
                                rust_decimal::Decimal::from_f32(rule.amount)
                                    .unwrap()
                                    .round_dp_with_strategy(
                                        2,
                                        rust_decimal::RoundingStrategy::ToNegativeInfinity,
                                    )
                            };
                            sum -= amount;
                            if amount.is_zero() {
                                continue;
                            }
                            builder.domestic(Type::DomesticTransaction {
                                account_from: info.account_id.clone(),
                                currency: info.currency.clone(),
                                amount,
                                account_to: rule.target_account.clone(),
                                bank_code: format!("{}", rule.target_bank.clone().unwrap()),
                                ks: rule.ks.map(|v| format!("{}", v)),
                                vs: rule.vs.map(|v| format!("{}", v)),
                                ss: rule.ss.map(|v| format!("{}", v)),
                                date: format!("{}", time::OffsetDateTime::now_utc().date()),
                                message_for_recipient: rule.message.clone(),
                                comment: rule.comment.clone(),
                                payment_reason: None,
                                payment_type: Some(format!("{}", rule.payment_type)),
                            });
                        };
                    }
                    if rules.len() > 0 {
                        info!("{:?}", builder.build());
                        info!("Post transaction");
                        match fio.import_transactions(builder.build()).await {
                            Ok(v) => {
                                info!("Transaction posted: {:?}", v);
                            }
                            Err(e) => {
                                error!("Failed to post transaction: {:?}", e);
                            }
                        };
                    }
                }
            }
            Err(e) => {
                error!("Failed to get newest account movements: {:?}", e);
            }
        }
    }
}
