use crate::entities::product::Product;
use crate::repositories::product_repository::ProductRepository;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct ProductService {
    pool: SqlitePool,
}

impl ProductService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn search_products_by_name(&self, name: &str) -> Vec<Product> {
        ProductRepository::search_by_name(&self.pool, name)
            .await
            .unwrap_or(vec![])
    }
}
