pub mod subcommands;

use crate::schema::account_tokens;
use crate::schema::accounts;
use diesel::{
    ExpressionMethods, Identifiable, NullableExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
};
use tracing::error;

pub fn add(name: &str, number: &i32, token: &str, read_only: &bool) {
    let mut conn = crate::establish_connection();
    diesel::insert_into(accounts::table)
        .values(&(accounts::number.eq(number), accounts::name.eq(name)))
        .on_conflict(accounts::number)
        .do_update()
        .set(accounts::name.eq(name))
        .execute(&mut conn)
        .expect("Error saving new account");
    diesel::insert_into(account_tokens::table)
        .values((
            account_tokens::account.eq(number),
            account_tokens::token.eq(token),
            account_tokens::read_only.eq(i32::from(*read_only)),
        ))
        .on_conflict((account_tokens::account, account_tokens::read_only))
        .do_update()
        .set(account_tokens::token.eq(token))
        .execute(&mut conn)
        .expect("Error saving new account token");
    println!("Account {name} added");
}

#[derive(Identifiable, Queryable)]
#[diesel(belongs_to(Accounts))]
#[diesel(table_name = accounts)]
#[diesel(primary_key(name))]
pub struct Account {
    pub number: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable)]
#[diesel(belongs_to(AccountToken))]
#[diesel(table_name = account_tokens)]
#[diesel(primary_key(id))]
pub struct Token {
    pub id: i32,
    pub account: i32,
    pub token: String,
    pub read_only: i32,
}

pub fn list(show_tokens: bool) {
    let mut conn = crate::establish_connection();
    let results = accounts::table
        .left_join(account_tokens::table)
        .select((
            accounts::all_columns,
            account_tokens::all_columns.nullable(),
        ))
        .load::<(Account, Option<Token>)>(&mut conn)
        .expect("Error loading accounts");
    println!("Displaying {} accounts", results.len());
    for (account, token) in results {
        print!("{} - {}", account.name, account.number);
        if let Some(token) = token {
            let t = token.token;
            if show_tokens {
                print!("\tToken #{}: {t}", token.id);
            } else {
                print!(
                    "\tToken #{}: {}..{}",
                    token.id,
                    t[..3].to_string(),
                    t[t.len() - 3..].to_string()
                );
            }
            println!("\tRead only: {}", token.read_only > 0);
        }
    }
}

pub fn get_token(account: &str, read_only: bool) -> Option<String> {
    let mut conn = crate::establish_connection();
    let result = accounts::table
        .inner_join(account_tokens::table)
        .filter(accounts::name.eq(account))
        .filter(account_tokens::read_only.eq(i32::from(read_only)))
        .select(account_tokens::token)
        .first::<String>(&mut conn);
    match result {
        Ok(token) => Some(token),
        Err(e) => {
            error!("Could not get token: {}", e);
            None
        }
    }
}

pub fn get_account_by_name(name: &str) -> Option<Account> {
    let mut conn = crate::establish_connection();
    let result = accounts::table
        .filter(accounts::name.eq(name))
        .first::<Account>(&mut conn);
    match result {
        Ok(account) => Some(account),
        Err(e) => {
            error!("Could not get account: {}", e);
            None
        }
    }
}

pub fn get_account_by_number(number: &i32) -> Option<Account> {
    let mut conn = crate::establish_connection();
    let result = accounts::table
        .filter(accounts::number.eq(number))
        .first::<Account>(&mut conn);
    match result {
        Ok(account) => Some(account),
        Err(e) => {
            error!("Could not get account: {}", e);
            None
        }
    }
}

pub fn remove(account: &str) {
    let mut conn = crate::establish_connection();
    let acc = get_account_by_name(account).unwrap();
    diesel::delete(accounts::table.filter(accounts::name.eq(account)))
        .execute(&mut conn)
        .expect("Error deleting account");
    diesel::delete(account_tokens::table.filter(account_tokens::account.eq(acc.number)))
        .execute(&mut conn)
        .expect("Error deleting account token");
    println!("Account {account} removed");
}
