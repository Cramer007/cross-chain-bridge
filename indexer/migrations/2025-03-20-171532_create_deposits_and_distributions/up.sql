-- Your SQL goes here
CREATE TABLE deposits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token TEXT NOT NULL,
    sender TEXT NOT NULL,
    recipient TEXT NOT NULL,
    amount TEXT NOT NULL,
    nonce INTEGER NOT NULL,
    processed BOOLEAN DEFAULT FALSE
);

CREATE TABLE distributions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token TEXT NOT NULL,
    sender TEXT NOT NULL,
    recipient TEXT NOT NULL,
    amount TEXT NOT NULL,
    nonce INTEGER NOT NULL,
    processed BOOLEAN DEFAULT FALSE
);
