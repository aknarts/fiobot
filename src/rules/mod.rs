use crate::schema::rules;
use diesel::{ExpressionMethods, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use prettytable::{format, row};

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

#[derive(Identifiable, Queryable)]
#[diesel(belongs_to(Rules))]
#[diesel(table_name = rules)]
#[diesel(primary_key(id))]
pub struct Rule {
    pub id: i32,
    pub account: i32,
    pub amount: f32,
    pub percent: i32,
    pub target_account: String,
    pub target_bank: Option<String>,
    pub bic: Option<String>,
    pub ks: Option<i32>,
    pub vs: Option<i32>,
    pub ss: Option<i32>,
    pub message: Option<String>,
    pub comment: Option<String>,
    pub for_: Option<String>,
    pub payment_type: i32,
    pub active: i32,
    pub sequence: i32,
}

pub fn list() {
    let mut conn = crate::establish_connection();
    let result = rules::table
        .select(rules::all_columns)
        .load::<Rule>(&mut conn)
        .expect("Error getting rules");
    let mut table = prettytable::Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row![
        "ID",
        "Account",
        "Amount",
        "Percent",
        "Target account",
        "Target bank",
        "BIC",
        "KS",
        "VS",
        "SS",
        "Message",
        "Comment",
        "For",
        "Payment type",
        "Active",
        "Sequence"
    ]);
    for rule in result {
        table.add_row(row![
            color -> rule.id,
            rule.account,
            rule.amount,
            rule.percent,
            rule.target_account,
            rule.target_bank.unwrap_or("".to_string()),
            rule.bic.unwrap_or("".to_string()),
            rule.ks.unwrap_or(0),
            rule.vs.unwrap_or(0),
            rule.ss.unwrap_or(0),
            rule.message.unwrap_or("".to_string()),
            rule.comment.unwrap_or("".to_string()),
            rule.for_.unwrap_or("".to_string()),
            rule.payment_type,
            rule.active,
            rule.sequence
        ]);
    }
    table.printstd();
}

pub fn remove(id: &i32) {
    let mut conn = crate::establish_connection();
    diesel::delete(rules::table.filter(rules::id.eq(id)))
        .execute(&mut conn)
        .expect("Error removing rule");
}
