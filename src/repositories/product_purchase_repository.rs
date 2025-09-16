use crate::entities::product_purchase::ProductPurchase;
use anyhow::Result;
use sqlx::Sqlite;

#[derive(Debug)]
pub struct ProductPurchaseRepository;

impl ProductPurchaseRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        product_purchase: &ProductPurchase,
    ) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO tb_product_purchase (product_id, purchase_id, price, quantity, total, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            product_purchase.product_id,
            product_purchase.purchase_id,
            product_purchase.price,
            product_purchase.quantity,
            product_purchase.total,
            product_purchase.created_at
        )
        .fetch_one(executor)
        .await?;

        Ok(rec.id.unwrap_or(0))
    }
}
