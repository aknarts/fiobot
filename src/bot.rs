use crate::account::get_accounts;
use fiocz_rs::types::account_statement::{Info, TransactionDataEnum};
use fiocz_rs::types::transaction::{ImportBuilder, Type};
use lettre::Transport;
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
                    let original_sum = sum.clone();
                    let mut builder = ImportBuilder::new();
                    let rules = crate::rules::get_rules_for_account(account.number);
                    for rule in &rules {
                        if rule.bic.is_none() {
                            let amount = if rule.percent > 0 {
                                (sum * rust_decimal::Decimal::from_i32(rule.amount).unwrap()
                                    / rust_decimal::Decimal::new(100, 0))
                                .round_dp_with_strategy(
                                    2,
                                    rust_decimal::RoundingStrategy::ToNegativeInfinity,
                                )
                            } else {
                                rust_decimal::Decimal::from_i32(rule.amount)
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
                                bank_code: rule.target_bank.clone().unwrap(),
                                ks: rule.ks.map(|v| format!("{v}")),
                                vs: rule.vs.map(|v| format!("{v}")),
                                ss: rule.ss.map(|v| format!("{v}")),
                                date: format!("{}", time::OffsetDateTime::now_utc().date()),
                                message_for_recipient: rule.message.clone(),
                                comment: rule.comment.clone(),
                                payment_reason: None,
                                payment_type: Some(format!("{}", rule.payment_type)),
                            });
                        };
                    }
                    if !rules.is_empty() {
                        info!("{:?}", builder.build());
                        info!("Post transaction");
                        match fio.import_transactions(builder.build()).await {
                            Ok(v) => {
                                info!("Transaction posted: {:?}", v);
                                if let Some(to) =
                                    crate::emails::get_email_by_account(account.number)
                                {
                                    send_email(to, info, original_sum);
                                }
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

fn send_email(to: String, info: Info, original_sum: rust_decimal::Decimal) {
    let from_email = std::env::var("FROM_EMAIL").expect("FROM_EMAIL must be set");
    let email = lettre::Message::builder()
        .from(from_email.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Fio transactions pending")
        .multipart(
            lettre::message::MultiPart::alternative()
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_PLAIN)
                        .body(format!(
                            r#"There are transactions pending for {}
Total value: {} {}"#,
                            info.account_id, original_sum, info.currency
                        )),
                )
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_HTML)
                        .body(format!(
                            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Fio transactions pending!</title>
</head>
<body>
    <div style="display: flex; flex-direction: column; align-items: left;">
        <p style="font-family: Arial, Helvetica, sans-serif;">There are transactions pending for {}</p>
        <p style="font-family: Arial, Helvetica, sans-serif;">Total value: {} {}</p>
    </div>
</body>
</html>"#,
                            info.account_id, original_sum, info.currency
                        )),
                ),
        ).expect("Failed to build email");

    let mailer = lettre::SendmailTransport::new();
    match mailer.send(&email) {
        Ok(_) => {
            info!("Email sent");
        }
        Err(e) => {
            error!("Failed to send email: {:?}", e);
        }
    }
}
