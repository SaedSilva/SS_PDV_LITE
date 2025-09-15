use sqlx::{Pool, Sqlite};

pub struct ProductPurchaseRepository {
    poll: Pool<Sqlite>
}