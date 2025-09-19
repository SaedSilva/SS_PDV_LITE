use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct ProductSale {
    pub id: i64,
    pub product_id: i64,
    pub sale_id: i64,
    pub price: i64,
    pub quantity: i64,
    pub total: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl ProductSale {
    pub fn new(
        id: i64,
        product_id: i64,
        sale_id: i64,
        price: i64,
        quantity: i64,
        total: i64,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            product_id,
            sale_id,
            price,
            quantity,
            total,
            created_at,
            updated_at: None,
        }
    }
}
