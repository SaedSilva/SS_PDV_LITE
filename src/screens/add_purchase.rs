use crate::helpers::{format_int_to_decimal, validate_float, validate_float_range, validate_int};
use crate::services::product_purchase_service::ProductPurchaseService;
use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Task};
use std::sync::Arc;

const ID_WIDTH: f32 = 50.0;
const EAN_WIDTH: f32 = 100.0;
static NAME_WIDTH: Length = Length::Fill;
const QNTD_WIDTH: f32 = 70.0;
const PRICE_UNIT_WIDTH: f32 = 100.0;
const PRICE_SALE_WIDTH: f32 = 100.0;
const PERCENTUAL_WIDTH: f32 = 100.0;
const TOTAL_WIDTH: f32 = 120.0;
const TOTAL_SALE_WIDTH: f32 = 120.0;

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
                percentual: "30,0".to_string(),
                total: "R$ 0,00".to_string(),
                total_sale: "R$ 0,00".to_string(),
            }],
            total: "R$ 0,00".to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![self.product_list(),].spacing(16).into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EanChange(index, value) => {
                if validate_int(&value) {
                    if let Some(product) = self.products.get_mut(index) {
                        product.ean = Some(value);
                    }
                }
            }
            Message::NameChange(index, value) => {
                if let Some(product) = self.products.get_mut(index) {
                    product.name = value.to_uppercase()
                }
            }
            Message::QuantityChange(index, value) => {
                if validate_int(&value) {
                    if let Some(product) = self.products.get_mut(index) {
                        product.quantity = value;
                        product.total = calculate_total(product);
                        product.total_sale = calculate_total_sale(product);
                    }
                }
            }
            Message::PriceUnitChange(index, value) => {
                if validate_float(&value) {
                    if let Some(product) = self.products.get_mut(index) {
                        product.price_unit = value;
                        product.total = calculate_total(product);
                        product.price_sale =
                            format!("{:.2}", calculate_price_sale(product)).replace(".", ",");
                        product.total_sale = calculate_total_sale(product);
                    }
                }
            }
            Message::PriceSaleChange(index, value) => {
                if validate_float(&value) {
                    if let Some(product) = self.products.get_mut(index) {
                        product.price_sale = value;
                        let percentual = calculate_percentual(product);
                        product.percentual = format!("{:.2}", percentual).replace(".", ",");
                        product.total_sale = calculate_total_sale(product);
                    }
                }
            }
            Message::PercentualChange(index, value) => {
                if validate_float_range(&value, 0.0, 100.0) {
                    if let Some(product) = self.products.get_mut(index) {
                        product.percentual = value;
                        let price_sale = calculate_price_sale(product);
                        product.price_sale = format!("{:.2}", price_sale).replace(".", ",");
                        product.total_sale = calculate_total_sale(product);
                    }
                }
            }

            Message::AddProduct => {
                self.products.push(ProductItem {
                    id: None,
                    ean: None,
                    name: "".to_string(),
                    quantity: "1".to_string(),
                    price_unit: "0".to_string(),
                    price_sale: "0".to_string(),
                    percentual: "30,0".to_string(),
                    total: "R$ 0,00".to_string(),
                    total_sale: "R$ 0,00".to_string(),
                });
            }
        }

        Task::none()
    }

    fn product_list(&self) -> Element<Message> {
        let mut list = column![
            row![
                text("ID").width(Length::Fixed(ID_WIDTH)),
                text("EAN").width(Length::Fixed(EAN_WIDTH)),
                text("PRODUTO").width(NAME_WIDTH),
                text("QNTD").width(Length::Fixed(QNTD_WIDTH)),
                text("P. UNIT.").width(Length::Fixed(PRICE_UNIT_WIDTH)),
                text("P. VENDA").width(Length::Fixed(PRICE_SALE_WIDTH)),
                text("%").width(Length::Fixed(PERCENTUAL_WIDTH)),
                text("TOTAL COMPRA").width(Length::Fixed(TOTAL_WIDTH)),
                text("TOTAL VENDA").width(Length::Fixed(TOTAL_SALE_WIDTH)),
            ]
            .spacing(16)
        ];

        for (index, product) in self.products.iter().enumerate() {
            list = list.push(
                row![
                    text(product.id.map_or("None".to_string(), |id| id.to_string())),
                    text_input("EAN", &product.ean.clone().unwrap_or_default())
                        .width(Length::Fixed(EAN_WIDTH))
                        .on_input(move |value| Message::EanChange(index, value)),
                    text_input("Nome", &product.name)
                        .width(NAME_WIDTH)
                        .on_input(move |value| Message::NameChange(index, value)),
                    text_input("Quantidade", &product.quantity)
                        .width(Length::Fixed(QNTD_WIDTH))
                        .on_input(move |value| Message::QuantityChange(index, value)),
                    text_input("Preço Unit.", &product.price_unit)
                        .width(Length::Fixed(PRICE_UNIT_WIDTH))
                        .on_input(move |value| Message::PriceUnitChange(index, value)),
                    text_input("Preço Venda", &product.price_sale)
                        .width(Length::Fixed(PRICE_SALE_WIDTH))
                        .on_input(move |value| Message::PriceSaleChange(index, value)),
                    text_input("Percentual", &product.percentual)
                        .width(Length::Fixed(PERCENTUAL_WIDTH))
                        .on_input(move |value| Message::PercentualChange(index, value)),
                    text(&product.total).width(Length::Fixed(TOTAL_WIDTH)),
                    text(&product.total_sale).width(Length::Fixed(TOTAL_SALE_WIDTH)),
                ]
                .align_y(Alignment::Center)
                .spacing(16),
            );
        }
        list = list.push(button("Adicionar Produto").on_press(Message::AddProduct));

        scrollable(list.spacing(8)).into()
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
    AddProduct,
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
    total_sale: String,
}

fn calculate_total(product: &ProductItem) -> String {
    let quantity = product.quantity.parse::<f64>().unwrap_or(0.0);
    let price_unit = product
        .price_unit
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    let total = (quantity * price_unit * 100.0) as i32;
    format_int_to_decimal(total)
}

fn calculate_price_sale(product: &ProductItem) -> f64 {
    let price_unit = product
        .price_unit
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    let percentual = product
        .percentual
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    price_unit + (price_unit * percentual / 100.0)
}

fn calculate_percentual(product: &ProductItem) -> f64 {
    let price_unit = product
        .price_unit
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    let price_sale = product
        .price_sale
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    if price_unit == 0.0 {
        0.0
    } else {
        ((price_sale - price_unit) / price_unit) * 100.0
    }
}

fn calculate_total_sale(product: &ProductItem) -> String {
    let quantity = product.quantity.parse::<f64>().unwrap_or(0.0);
    let price_sale = product
        .price_sale
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    let total_sale = (quantity * price_sale * 100.0) as i32;
    format_int_to_decimal(total_sale)
}
