-- Add migration script here
CREATE TABLE Product
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    price      INTEGER NOT NULL,
    quantity   INTEGER NOT NULL,
    ean        TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER
);

CREATE TABLE Sale
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    total      INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER
);

CREATE TABLE ProductSale
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL,
    sale_id    INTEGER NOT NULL,
    quantity   INTEGER NOT NULL,
    price      INTEGER NOT NULL,
    total      INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER,
    FOREIGN KEY (product_id) REFERENCES Product (id),
    FOREIGN KEY (sale_id) REFERENCES Sale (id)
);

CREATE TABLE Purchase
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    total      INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER
);

CREATE TABLE ProductPurchase
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id  INTEGER NOT NULL,
    purchase_id INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,
    price       INTEGER NOT NULL,
    total       INTEGER NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER,
    FOREIGN KEY (product_id) REFERENCES Product (id),
    FOREIGN KEY (purchase_id) REFERENCES Purchase (id)
);