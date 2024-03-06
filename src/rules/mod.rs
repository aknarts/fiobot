use crate::schema::rules;
use diesel::{ExpressionMethods, Identifiable, QueryDsl, Queryable, RunQueryDsl};
use prettytable::{format, row, Table};

pub mod subcommands;

pub fn add(
    account: &i64,
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

#[derive(Identifiable, Queryable, Debug, PartialEq, Clone)]
#[diesel(belongs_to(Rules))]
#[diesel(table_name = rules)]
#[diesel(primary_key(id))]
pub struct Rule {
    pub id: i32,
    pub account: i64,
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

pub fn list(account: Option<&i64>) {
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
    match account {
        None => {
            for account in crate::account::get_accounts() {
                while table.len() > 0 {
                    table.remove_row(0);
                }
                println!("Account: {}", account.number);
                let result = get_rules_for_account(&account.number);
                build_table_row(&mut table, result);
                table.printstd();
            }
        }
        Some(account) => {
            let result = get_rules_for_account(account);
            build_table_row(&mut table, result);
            table.printstd();
        }
    }
}

fn build_table_row(table: &mut Table, result: Vec<Rule>) {
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
}

pub fn get_rules_for_account(account: &i64) -> Vec<Rule> {
    let mut conn = crate::establish_connection();
    rules::table
        .filter(rules::account.eq(account))
        .order(rules::sequence)
        .load::<Rule>(&mut conn)
        .expect("Error getting rules for account")
}

pub fn edit(
    id: &i32,
    account: Option<&i64>,
    amount: Option<&i32>,
    target_account: Option<&String>,
    target_bank: Option<&String>,
    bic: Option<&String>,
    ks: Option<&i32>,
    vs: Option<&i32>,
    ss: Option<&i32>,
    message: Option<&String>,
    comment: Option<&String>,
    for_: Option<&String>,
    payment_type: Option<&i32>,
    percent: bool,
    sequence: Option<&i32>,
) -> Result<(), &'static str> {
    let mut conn = crate::establish_connection();
    if account.is_some() {
        let account_verified = match crate::account::get_account_by_number(account.unwrap()) {
            None => return Err("Account not found"),
            Some(a) => a.number,
        };
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::account.eq(account_verified))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if amount.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set((
                rules::amount.eq(*amount.unwrap() as f32),
                rules::percent.eq(percent as i32),
            ))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if target_account.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::target_account.eq(target_account.unwrap()))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if target_bank.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::target_bank.eq(target_bank))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if bic.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::bic.eq(bic))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if ks.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::ks.eq(ks))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if vs.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::vs.eq(vs))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if ss.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::ss.eq(ss))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if message.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::message.eq(message))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if comment.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::comment.eq(comment))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if for_.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::for_.eq(for_))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if payment_type.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::payment_type.eq(payment_type.unwrap()))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    if sequence.is_some() {
        diesel::update(rules::table)
            .filter(rules::id.eq(id))
            .set(rules::sequence.eq(sequence.unwrap()))
            .execute(&mut conn)
            .expect("Error editing rule");
    }
    Ok(())
}

pub fn remove(id: &i32) {
    let mut conn = crate::establish_connection();
    diesel::delete(rules::table.filter(rules::id.eq(id)))
        .execute(&mut conn)
        .expect("Error removing rule");
}

pub fn toggle(id: &i32) {
    let mut conn = crate::establish_connection();
    let rule = rules::table
        .filter(rules::id.eq(id))
        .first::<Rule>(&mut conn)
        .expect("Error getting rule");
    let new_active = if rule.active == 0 { 1 } else { 0 };
    diesel::update(rules::table)
        .filter(rules::id.eq(id))
        .set(rules::active.eq(new_active))
        .execute(&mut conn)
        .expect("Error toggling rule");
}
