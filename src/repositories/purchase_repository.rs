use crate::entities::purchase::Purchase;
use anyhow::Result;
use sqlx::Sqlite;

#[derive(Debug)]
pub struct PurchaseRepository;

impl PurchaseRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        purchase: &Purchase,
    ) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO tb_purchase (total, created_at)
            VALUES (?, ?)
            RETURNING id
            "#,
            purchase.total,
            purchase.created_at
        )
        .fetch_one(executor)
        .await?;

        Ok(rec.id)
    }

    pub async fn find_by_id<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        id: i64,
    ) -> Result<Option<Purchase>> {
        let entity = sqlx::query_as!(
            Purchase,
            "
            SELECT id, total, created_at, updated_at
            FROM tb_purchase
            WHERE id = ?
            ",
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(entity)
    }
}
