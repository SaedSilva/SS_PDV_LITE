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
        id: i64,
        name: String,
        price_sale: i64,
        price_purchase: i64,
        quantity: i64,
        ean: Option<String>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            name,
            price_sale,
            price_purchase,
            quantity,
            ean,
            created_at,
            updated_at: None,
        }
    }
}
