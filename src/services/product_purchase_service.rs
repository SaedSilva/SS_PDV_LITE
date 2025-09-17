use crate::entities::product::Product;
use crate::entities::product_purchase::ProductPurchase;
use crate::entities::purchase::Purchase;
use crate::repositories::product_purchase_repository::ProductPurchaseRepository;
use crate::repositories::product_repository::ProductRepository;
use crate::repositories::purchase_repository::PurchaseRepository;
use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct ProductPurchaseService {
    pool: SqlitePool,
}

impl ProductPurchaseService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_purchase(&self, products: Vec<Product>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now().naive_local();
        let total = products.iter().map(|p| p.price_purchase * p.quantity).sum();
        let purchase_id =
            PurchaseRepository::insert(&mut *tx, &Purchase::new(0, total, now)).await?;

        for product in products.into_iter().as_mut_slice() {
            let product_id = if product.id == 0 {
                ProductRepository::insert(&mut *tx, product).await?
            } else if let Some(existing_product) =
                ProductRepository::find_by_id(&mut *tx, product.id).await?
            {
                product.quantity += existing_product.quantity;
                ProductRepository::update(&mut *tx, product).await?;
                product.id
            } else {
                return Err(anyhow!("Product with ID {} not found", product.id));
            };
            ProductPurchaseRepository::insert(
                &mut *tx,
                &ProductPurchase::new(
                    0,
                    product_id,
                    purchase_id,
                    product.quantity,
                    product.price_purchase,
                    product.quantity * product.price_purchase,
                    now,
                ),
            )
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
