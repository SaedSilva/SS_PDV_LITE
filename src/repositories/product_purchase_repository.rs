use sqlx::{Pool, Sqlite};

struct ProductPurchaseRepository {
    poll: Pool<Sqlite>
}