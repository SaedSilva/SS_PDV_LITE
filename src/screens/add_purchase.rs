use crate::services::product_purchase_service::ProductPurchaseService;
use iced::widget::{column, row, text, text_input};
use iced::{Element, Task};
use std::sync::Arc;

#[derive(Debug)]
pub struct State {
    product_purchase_service: Arc<ProductPurchaseService>,
    products: Vec<ProductItem>,
    total: String,
}

impl State {
    pub fn new(product_purchase_service: Arc<ProductPurchaseService>) -> Self {
        Self {
            product_purchase_service,
            products: vec![ProductItem {
                id: None,
                ean: None,
                name: "".to_string(),
                quantity: "1".to_string(),
                price_unit: "0".to_string(),
                price_sale: "0".to_string(),
                percentual: "0".to_string(),
                total: "0".to_string(),
            }],
            total: "".to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            self.product_list(),
        ].spacing(16).into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EanChange(index, value) => {
                if let Some(product) = self.products.get_mut(index as usize) {
                    product.ean = Some(value);
                }
            }
            Message::NameChange(_, _) => {}
            Message::QuantityChange(_, _) => {}
            Message::PriceUnitChange(_, _) => {}
            Message::PriceSaleChange(_, _) => {}
            Message::PercentualChange(_, _) => {}
        }

        Task::none()
    }

    fn product_list(&self) -> Element<Message> {
        let mut list = column![
            row![
                text("ID"),
                text("EAN"),
                text("PRODUTO"),
                text("QNTD"),
                text("P. UNIT."),
                text("P. VENDA"),
                text("PERCENTUAL"),
                text("TOTAL"),
            ]
            .spacing(16)
        ];

        for (index, product) in self.products.iter().enumerate() {
            list = list.push(
                row![
                    text(product.id.map_or("".to_string(), |id| id.to_string())),
                    text_input("EAN", &product.ean.clone().unwrap_or_default())
                        .on_input(move |value| Message::EanChange(index, value)),
                    text_input("Nome", &product.name)
                        .on_input(move |value| Message::NameChange(index, value)),
                    text_input("Quantidade", &product.quantity)
                        .on_input(move |value| Message::QuantityChange(index, value)),
                    text_input("Preço Unit.", &product.price_unit)
                        .on_input(move |value| Message::PriceUnitChange(index, value)),
                    text_input("Preço Venda", &product.price_sale)
                        .on_input(move |value| Message::PriceSaleChange(index, value)),
                    text_input("Percentual", &product.percentual)
                        .on_input(move |value| Message::PercentualChange(index, value)),
                    text(&product.total),
                ]
                .spacing(16),
            );
        }

        list.into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EanChange(usize, String),
    NameChange(usize, String),
    QuantityChange(usize, String),
    PriceUnitChange(usize, String),
    PriceSaleChange(usize, String),
    PercentualChange(usize, String),
}

#[derive(Debug, Clone, Default)]
struct ProductItem {
    id: Option<i64>,
    ean: Option<String>,
    name: String,
    quantity: String,
    price_unit: String,
    price_sale: String,
    percentual: String,
    total: String,
}
