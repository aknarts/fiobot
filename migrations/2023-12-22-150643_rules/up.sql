-- Your SQL goes here
CREATE TABLE rules
(
    id             INTEGER NOT NULL
        CONSTRAINT rules_pk
            PRIMARY KEY AUTOINCREMENT,
    account        BIG INT NOT NULL
        CONSTRAINT rules_accounts_number_fk
            REFERENCES accounts,
    amount         INTEGER NOT NULL,
    percent        INTEGER NOT NULL,
    target_account TEXT    NOT NULL,
    target_bank    TEXT,
    bic            TEXT,
    ks             INTEGER,
    vs             INTEGER,
    ss             INTEGER,
    message        TEXT,
    comment        TEXT,
    for            TEXT,
    payment_type   INTEGER NOT NULL,
    active         INTEGER NOT NULL,
    sequence       INTEGER NOT NULL
);

