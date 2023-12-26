// @generated automatically by Diesel CLI.

diesel::table! {
    account_tokens (id) {
        id -> Integer,
        account -> Integer,
        token -> Text,
        read_only -> Integer,
    }
}

diesel::table! {
    accounts (number) {
        number -> Integer,
        name -> Text,
    }
}

diesel::table! {
    rules (id) {
        id -> Integer,
        account -> Integer,
        amount -> Float,
        percent -> Integer,
        target_account -> Integer,
        target_bank -> Nullable<Integer>,
        bic -> Nullable<Text>,
        ks -> Nullable<Integer>,
        vs -> Nullable<Integer>,
        ss -> Nullable<Integer>,
        message -> Nullable<Text>,
        comment -> Nullable<Text>,
        #[sql_name = "for"]
        for_ -> Nullable<Text>,
        payment_type -> Integer,
        active -> Integer,
        sequence -> Integer,
    }
}

diesel::joinable!(account_tokens -> accounts (account));
diesel::joinable!(rules -> accounts (account));

diesel::allow_tables_to_appear_in_same_query!(
    account_tokens,
    accounts,
    rules,
);
