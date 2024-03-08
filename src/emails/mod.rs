use crate::schema::{accounts, emails};
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};
use tracing::error;

pub mod subcommands;

pub(crate) fn add(
    account: Option<&i64>,
    email: &String,
    default: bool,
) -> Result<(), &'static str> {
    let mut conn = crate::establish_connection();
    let acc_num = if !default {
        match account {
            None => return Err("You must specify an account"),
            Some(account) => match crate::account::get_account_by_number(*account) {
                None => return Err("Account not found"),
                Some(a) => a.number,
            },
        }
    } else {
        0i64
    };
    diesel::insert_into(emails::table)
        .values((emails::account.eq(acc_num), emails::email.eq(email)))
        .on_conflict(emails::account)
        .do_update()
        .set(emails::email.eq(email))
        .execute(&mut conn)
        .expect("Error adding email");
    Ok(())
}

pub(crate) fn remove(account: i64) {
    if account == 0 {
        return;
    }
    let mut conn = crate::establish_connection();
    diesel::delete(emails::table.filter(emails::account.eq(account)))
        .execute(&mut conn)
        .expect("Error deleting email");
    println!("Email removed");
}

pub(crate) fn list() {
    let mut conn = crate::establish_connection();
    let results = emails::table
        .inner_join(accounts::table)
        .select((emails::email, accounts::all_columns))
        .load::<(String, crate::account::Account)>(&mut conn)
        .expect("Error loading emails");
    println!("Displaying {} emails", results.len());
    for (email, account) in results {
        if account.number == 0 {
            println!("Default email: {}", email);
            continue;
        } else {
            println!("{}({}): {}", account.name, account.number, email);
        }
    }
    println!(
        "Email for 2100299408: {:?}",
        get_email_by_account(2100299408)
    );
}

pub(crate) fn get_email_by_account(account: i64) -> Option<String> {
    let mut conn = crate::establish_connection();
    let result = emails::table
        .filter(emails::account.eq(account))
        .or_filter(emails::account.eq(0))
        .select(emails::email)
        .order(emails::account.desc())
        .first::<String>(&mut conn);
    match result {
        Ok(email) => Some(email),
        Err(e) => {
            error!("Could not get email: {}", e);
            None
        }
    }
}
