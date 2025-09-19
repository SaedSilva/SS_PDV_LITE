use crate::entities::product::Product;
use crate::entities::product_sale::ProductSale;
use crate::entities::sale::Sale;
use crate::repositories::product_repository::ProductRepository;
use crate::repositories::product_sale_repository::ProductSaleRepository;
use crate::repositories::sale_repository::SaleRepository;
use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct ProductSaleService {
    pool: SqlitePool,
}

impl ProductSaleService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_sale(&self, products: Vec<Product>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now().naive_local();
        let total = products.iter().map(|p| p.price_sale * p.quantity).sum();
        let sale_id = SaleRepository::insert(&mut *tx, &Sale::new(0, total, now)).await?;

        for product in products.into_iter().as_mut_slice() {
            let product_id = if product.id == 0 {
                ProductRepository::insert(&mut *tx, product).await?
            } else if let Some(existing_product) =
                ProductRepository::find_by_id(&mut *tx, product.id).await?
            {
                product.quantity = existing_product.quantity - product.quantity;
                if product.quantity < 0 {
                    return Err(anyhow!("Insufficient stock for product ID {}", product.id));
                }
                ProductRepository::update(&mut *tx, product).await?;
                product.id
            } else {
                return Err(anyhow!("Product with ID {} not found", product.id));
            };
            ProductSaleRepository::insert(
                &mut *tx,
                &ProductSale::new(
                    0,
                    product_id,
                    sale_id,
                    product.price_sale,
                    product.quantity,
                    product.quantity * product.price_sale,
                    now,
                ),
            )
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
