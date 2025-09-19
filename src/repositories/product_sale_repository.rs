
use anyhow::Result;
use sqlx::Sqlite;
use crate::entities::product_sale::ProductSale;

#[derive(Debug)]
pub struct ProductSaleRepository;

impl ProductSaleRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        entity: &ProductSale,
    ) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO tb_product_sale (product_id, sale_id, price, quantity, total, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            entity.product_id,
            entity.sale_id,
            entity.price,
            entity.quantity,
            entity.total,
            entity.created_at
        )
        .fetch_one(executor)
        .await?;

        Ok(rec.id.unwrap_or(0))
    }
}
