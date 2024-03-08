-- Your SQL goes here
CREATE TABLE emails
(
    id      INTEGER NOT NULL
        CONSTRAINT rules_pk
            PRIMARY KEY AUTOINCREMENT,
    account BIG INT NOT NULL
        CONSTRAINT rules_accounts_number_fk
            REFERENCES accounts,
    email   TEXT    NOT NULL,
    CONSTRAINT emails_unique
        UNIQUE (account)
);