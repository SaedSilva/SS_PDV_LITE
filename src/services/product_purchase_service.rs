use crate::repositories::product_purchase_repository::ProductPurchaseRepository;
use crate::repositories::product_repository::ProductRepository;

#[derive(Debug)]
pub struct ProductPurchaseService {
    product_purchase_repository: ProductPurchaseRepository,
    product_repository: ProductRepository,
}

impl ProductPurchaseService {
    pub fn new(
        product_purchase_repository: ProductPurchaseRepository,
        product_repository: ProductRepository,
    ) -> Self {
        Self {
            product_purchase_repository,
            product_repository,
        }
    }
}
