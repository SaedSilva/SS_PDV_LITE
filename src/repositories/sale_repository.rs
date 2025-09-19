use crate::entities::purchase::Purchase;
use anyhow::Result;
use sqlx::Sqlite;
use crate::entities::sale::Sale;

#[derive(Debug)]
pub struct SaleRepository;

impl SaleRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        entity: &Sale,
    ) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO tb_sale (total, created_at)
            VALUES (?, ?)
            RETURNING id
            "#,
            entity.total,
            entity.created_at
        )
        .fetch_one(executor)
        .await?;

        Ok(rec.id)
    }

    pub async fn find_by_id<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        id: i64,
    ) -> Result<Option<Sale>> {
        let entity = sqlx::query_as!(
            Sale,
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
