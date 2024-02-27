use crate::schema::rules;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub mod subcommands;

pub fn add(
    account: &i32,
    amount: &i32,
    target_account: &str,
    target_bank: Option<&String>,
    bic: Option<&String>,
    ks: Option<&i32>,
    vs: Option<&i32>,
    ss: Option<&i32>,
    message: Option<&String>,
    comment: Option<&String>,
    for_: Option<&String>,
    payment_type: &i32,
    active: bool,
    percent: bool,
    sequence: Option<&i32>,
) -> Result<(), &'static str> {
    let mut conn = crate::establish_connection();
    let account_verified = match crate::account::get_account_by_number(account) {
        None => return Err("Account not found"),
        Some(a) => a.number,
    };
    let sequence_verified = match sequence {
        None => {
            let max_sequence = rules::table
                .select(diesel::dsl::max(rules::sequence))
                .filter(rules::account.eq(account_verified))
                .first::<Option<i32>>(&mut conn)
                .expect("Error getting max sequence");
            match max_sequence {
                None => 0,
                Some(s) => s + 10,
            }
        }
        Some(s) => *s,
    };
    diesel::insert_into(rules::table)
        .values((
            rules::account.eq(account_verified),
            rules::amount.eq(*amount as f32),
            rules::target_account.eq(target_account),
            rules::target_bank.eq(target_bank),
            rules::bic.eq(bic),
            rules::ks.eq(ks),
            rules::vs.eq(vs),
            rules::ss.eq(ss),
            rules::message.eq(message),
            rules::comment.eq(comment),
            rules::for_.eq(for_),
            rules::payment_type.eq(*payment_type),
            rules::active.eq(active as i32),
            rules::percent.eq(percent as i32),
            rules::sequence.eq(sequence_verified),
        ))
        .execute(&mut conn)
        .expect("Error adding rule");
    Ok(())
}
