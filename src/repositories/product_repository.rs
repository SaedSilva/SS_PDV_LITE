use crate::entities::product::Product;
use anyhow::Result;
use chrono::Utc;
use sqlx::{Sqlite, SqlitePool};

#[derive(Debug)]
pub struct ProductRepository;

impl ProductRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        product: &Product,
    ) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO tb_product (name, price_sale, price_purchase, quantity, ean, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id
            "#,
            product.name,
            product.price_sale,
            product.price_purchase,
            product.quantity,
            product.ean,
            product.created_at
        )
        .fetch_one(executor)
        .await?;

        Ok(rec.id)
    }

    pub async fn update<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        product: &Product,
    ) -> Result<()> {
        let updated_at = Utc::now().naive_local();
        sqlx::query!(
            r#"
            UPDATE tb_product
            SET name = ?, price_sale = ?, price_purchase = ?, quantity = ?, ean = ?, updated_at = ?
            WHERE id = ?
            "#,
            product.name,
            product.price_sale,
            product.price_purchase,
            product.quantity,
            product.ean,
            updated_at,
            product.id
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn find_by_id<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        id: i64,
    ) -> Result<Option<Product>> {
        let product = sqlx::query_as!(
            Product,
            "
            SELECT id, name, price_sale, price_purchase, quantity, ean, created_at, updated_at
            FROM tb_product
            WHERE id = ?
            ",
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(product)
    }

    pub async fn search_by_name<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        name: &str,
    ) -> Result<Vec<Product>> {
        let name = format!("%{}%", name);
        let products = sqlx::query_as!(
            Product,
            "
            SELECT id, name, price_sale, price_purchase, quantity, ean, created_at, updated_at
            FROM tb_product
            WHERE quantity > 0 AND name LIKE ?
            LIMIT 10
            ",
            name
        )
        .fetch_all(executor)
        .await?;

        Ok(products)
    }

    pub async fn search_by_ean<'e, E: sqlx::Executor<'e, Database = Sqlite>>(
        executor: E,
        ean: &str,
    ) -> Result<Vec<Product>> {
        let product = sqlx::query_as!(
            Product,
            "
            SELECT id, name, price_sale, price_purchase, quantity, ean, created_at, updated_at
            FROM tb_product
            WHERE ean = ?
            LIMIT 10
            ",
            ean
        )
        .fetch_all(executor)
        .await?;

        Ok(product)
    }
}
