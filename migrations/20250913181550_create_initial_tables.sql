-- Add migration script here
CREATE TABLE Product
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    name     TEXT    NOT NULL,
    price    INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    ean      TEXT
);
