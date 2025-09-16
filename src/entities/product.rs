use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub quantity: i64,
    pub ean: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Product {
    pub fn new(name: String, price: i64, quantity: i64, ean: Option<String>) -> Self {
        Self {
            id: 0, // Will be set by the database
            name,
            price,
            quantity,
            ean,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
