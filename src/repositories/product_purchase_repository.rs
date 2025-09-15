use sqlx::{Pool, Sqlite};

#[derive(Debug)]
pub struct ProductPurchaseRepository {
    poll: Pool<Sqlite>
}

impl ProductPurchaseRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { poll: pool }
    }
}