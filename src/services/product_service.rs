use crate::entities::product::Product;
use crate::repositories::product_repository::ProductRepository;
use std::sync::Arc;

#[derive(Debug)]
pub struct ProductService {
    product_repository: Arc<ProductRepository>,
}

impl ProductService {
    pub fn new(product_repository: Arc<ProductRepository>) -> Self {
        Self { product_repository }
    }

    pub async fn search_products_by_name(&self, name: &str) -> Vec<Product> {
        self.product_repository
            .search_by_name(name)
            .await
            .unwrap_or(vec![])
    }
}
