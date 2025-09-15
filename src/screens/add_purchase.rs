use crate::services::product_purchase_service::ProductPurchaseService;
use iced::widget::{column, row, text};
use iced::{Element, Task};

#[derive(Debug)]
pub struct State {
    product_purchase_service: ProductPurchaseService,
    products: Vec<ProductItem>,
    total: String,
}

impl State {
    pub fn new(product_purchase_service: ProductPurchaseService) -> Self {
        Self {
            product_purchase_service,
            products: vec![],
            total: "".to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![].spacing(16).into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EanChange(_, _) => {}
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

        list.into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EanChange(i32, String),
    NameChange(i32, String),
    QuantityChange(i32, String),
    PriceUnitChange(i32, String),
    PriceSaleChange(i32, String),
    PercentualChange(i32, String),
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
