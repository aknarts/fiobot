pub mod subcommands;

pub fn add(
    account: &i32,
    amount: &i32,
    target_account: &str,
    target_bank: Option<&str>,
    bic: Option<&str>,
    ks: Option<&i32>,
    vs: Option<&i32>,
    ss: Option<&i32>,
    message: Option<&str>,
    comment: Option<&str>,
    for_: Option<&str>,
    payment_type: &i32,
    active: bool,
    percent: bool,
    sequence: Option<&i32>,
) {
    let mut conn = crate::establish_connection();

    println!("Adding rule");
}
