use crate::repositories::product_purchase_repository::ProductPurchaseRepository;
use crate::repositories::product_repository::ProductRepository;
use std::sync::Arc;
use crate::entities::product::Product;

#[derive(Debug)]
pub struct ProductPurchaseService {
    product_purchase_repository: Arc<ProductPurchaseRepository>,
    product_repository: Arc<ProductRepository>,
}

impl ProductPurchaseService {
    pub fn new(
        product_purchase_repository: Arc<ProductPurchaseRepository>,
        product_repository: Arc<ProductRepository>,
    ) -> Self {
        Self {
            product_purchase_repository,
            product_repository,
        }
    }

    pub async fn add_purchase(&self, products: Vec<Product>) {
        for product in products {
            if product.id == 0 {
                self.product_repository.insert(&product).await.unwrap();
            }
        }
    }
}
