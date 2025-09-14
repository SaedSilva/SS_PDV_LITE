-- Add migration script here
CREATE TABLE tb_product
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    price      INTEGER NOT NULL,
    quantity   INTEGER NOT NULL,
    ean        TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME
);

CREATE TABLE tb_sale
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    total      INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME
);

CREATE TABLE tb_product_sale
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id INTEGER NOT NULL,
    sale_id    INTEGER NOT NULL,
    quantity   INTEGER NOT NULL,
    price      INTEGER NOT NULL,
    total      INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME,
    FOREIGN KEY (product_id) REFERENCES tb_product (id),
    FOREIGN KEY (sale_id) REFERENCES tb_sale (id)
);

CREATE TABLE tb_purchase
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    total      INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME
);

CREATE TABLE tb_product_purchase
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    product_id  INTEGER NOT NULL,
    purchase_id INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,
    price       INTEGER NOT NULL,
    total       INTEGER NOT NULL,
    created_at  DATETIME NOT NULL,
    updated_at  DATETIME,
    FOREIGN KEY (product_id) REFERENCES tb_product (id),
    FOREIGN KEY (purchase_id) REFERENCES tb_purchase (id)
);