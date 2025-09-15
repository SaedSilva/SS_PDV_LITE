use crate::repositories::product_purchase_repository::ProductPurchaseRepository;
use crate::repositories::product_repository::ProductRepository;
use std::sync::Arc;

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
}
