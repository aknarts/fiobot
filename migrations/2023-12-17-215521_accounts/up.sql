-- Your SQL goes here
CREATE TABLE accounts
(
    number BIG INT NOT NULL
        CONSTRAINT accounts_pk
            PRIMARY KEY,
    name   TEXT    NOT NULL
);

INSERT INTO accounts (number, name)
VALUES (0, 'Default account');

CREATE TABLE account_tokens
(
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    account   BIG INT NOT NULL
        CONSTRAINT account_tokens_accounts_number_fk
            REFERENCES accounts
            ON DELETE CASCADE,
    token     TEXT    NOT NULL,
    read_only INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT account_tokens_unique
        UNIQUE (read_only, account)
);



