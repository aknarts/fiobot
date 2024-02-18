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
) {
    let mut conn = crate::establish_connection();

    println!("Adding rule");
}
