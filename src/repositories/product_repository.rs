use crate::entities::product::Product;
use anyhow::Result;
use sqlx::SqlitePool;

struct ProductRepository {
    pool: SqlitePool,
}

impl ProductRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, product: &Product) -> Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO tb_product (name, price, quantity, ean, created_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id
            "#,
            product.name,
            product.price,
            product.quantity,
            product.ean,
            product.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn search_by_name(&self, name: &str) -> Result<Vec<Product>> {
        let name = format!("%{}%", name);
        let products = sqlx::query_as!(
            Product,
            "
            SELECT id, name, price, quantity, ean, created_at, updated_at
            FROM tb_product
            WHERE name LIKE ?
            LIMIT 10
            ",
            name
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(products)
    }

    pub async fn search_by_ean(&self, ean: &str) -> Result<Vec<Product>> {
        let product = sqlx::query_as!(
            Product,
            "
            SELECT id, name, price, quantity, ean, created_at, updated_at
            FROM tb_product
            WHERE ean = ?
            LIMIT 10
            ",
            ean
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(product)
    }
}
