use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price_sale: i64,
    pub price_purchase: i64,
    pub quantity: i64,
    pub ean: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Product {
    pub fn new(
        name: String,
        price_sale: i64,
        price_purchase: i64,
        quantity: i64,
        ean: Option<String>,
    ) -> Self {
        Self {
            id: 0, // Will be set by the database
            name,
            price_sale,
            price_purchase,
            quantity,
            ean,
            created_at: Local::now().naive_local(),
            updated_at: None,
        }
    }
}
