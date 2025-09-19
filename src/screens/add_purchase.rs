use crate::entities::product::Product;
use crate::helpers::{
    f64_to_i64, format_int_to_decimal, i64_to_f64, validate_float, validate_float_range,
    validate_int,
};
use crate::services::product_purchase_service::ProductPurchaseService;
use crate::services::product_service::ProductService;
use chrono::Local;
use iced::widget::{button, column, horizontal_space, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length, Task};
use std::sync::Arc;

const REMOVE_BUTTON_WIDTH: f32 = 30.0;
const ID_WIDTH: f32 = 50.0;
const EAN_WIDTH: f32 = 100.0;
static NAME_WIDTH: Length = Length::Fill;
const QNTD_WIDTH: f32 = 70.0;
const PRICE_UNIT_WIDTH: f32 = 100.0;
const PRICE_SALE_WIDTH: f32 = 100.0;
const PERCENTUAL_WIDTH: f32 = 50.0;
const TOTAL_WIDTH: f32 = 120.0;
const TOTAL_SALE_WIDTH: f32 = 120.0;

#[derive(Debug)]
pub struct State {
    product_purchase_service: Arc<ProductPurchaseService>,
    product_service: Arc<ProductService>,
    products: Vec<ProductItem>,
    total: String,
    show_search: bool,
    search_index: Option<usize>,
    search_text: String,
    search_products: Vec<Product>,
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
    FinishPurchase,
    RemoveProduct(usize),
    SearchProduct(usize),
    CloseSearch,
    SearchTextChange(String),
    ChangeProductsSearch(Vec<Product>),
    ProductSelected(usize, Product),
}

impl State {
    pub fn new(
        product_purchase_service: Arc<ProductPurchaseService>,
        product_service: Arc<ProductService>,
    ) -> Self {
        Self {
            product_purchase_service,
            product_service,
            products: vec![ProductItem {
                id: None,
                ean: None,
                name: "".to_string(),
                quantity: "1".to_string(),
                price_unit: "0,00".to_string(),
                price_sale: "0,00".to_string(),
                percentual: "30,0".to_string(),
                total: "R$ 0,00".to_string(),
                total_sale: "R$ 0,00".to_string(),
            }],
            total: "R$ 0,00".to_string(),
            show_search: false,
            search_index: None,
            search_text: "".to_string(),
            search_products: vec![],
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if (self.show_search) {
            self.search()
        } else {
            column![
                self.product_list(),
                row![
                    button("ADICIONAR ITEM").on_press(Message::AddProduct),
                    horizontal_space(),
                    row![
                        text("TOTAL: "),
                        text(self.total().clone()),
                        button("FINALIZAR COMPRA").on_press(Message::FinishPurchase)
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center),
                ]
            ]
            .spacing(16)
            .align_x(Alignment::End)
            .into()
        }
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
                    price_unit: "0,00".to_string(),
                    price_sale: "0,00".to_string(),
                    percentual: "30,0".to_string(),
                    total: "R$ 0,00".to_string(),
                    total_sale: "R$ 0,00".to_string(),
                });
            }
            Message::RemoveProduct(index) => {
                if index < self.products.len() {
                    self.products.remove(index);
                }
            }
            Message::SearchProduct(index) => {
                self.show_search = true;
                self.search_index = Some(index);
                self.search_text = "".to_string();
                self.search_products = vec![];
            }
            Message::CloseSearch => {
                self.show_search = false;
                self.search_index = None;
                self.search_text = "".to_string();
                self.search_products = vec![];
            }
            Message::SearchTextChange(value) => {
                self.search_text = value.to_uppercase();
                if value.is_empty() {
                    self.search_products = vec![];
                } else {
                    let product_service = self.product_service.clone();
                    let search_text = self.search_text.clone();
                    return Task::perform(
                        async move { product_service.search_products_by_name(&search_text).await },
                        Message::ChangeProductsSearch,
                    );
                }
            }
            Message::ChangeProductsSearch(value) => {
                self.search_products = value;
            }
            Message::FinishPurchase => {
                let products = self.products.iter().map(|x| x.to_product()).collect();
                let product_purchase_service = self.product_purchase_service.clone();
                return Task::perform(
                    async move { product_purchase_service.add_purchase(products).await },
                    |_| Message::CloseSearch,
                );
            }
            Message::ProductSelected(index, value) => {
                if let Some(search_index) = self.search_index {
                    if let Some(product_item) = self.products.get_mut(search_index) {
                        *product_item = ProductItem::from_product(&value);
                        product_item.total = calculate_total(product_item);
                        product_item.total_sale = calculate_total_sale(product_item);
                    }
                }
                self.show_search = false;
                self.search_index = None;
                self.search_text = "".to_string();
                self.search_products = vec![];
            }
        }

        Task::none()
    }

    fn product_list(&self) -> Element<'_, Message> {
        let header = row![
            text("").width(Length::Fixed(REMOVE_BUTTON_WIDTH)),
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
        .spacing(16);

        let mut list_products = column![];
        for (index, product) in self.products.iter().enumerate() {
            list_products = list_products.push(
                row![
                    button("X")
                        .width(Length::Fixed(REMOVE_BUTTON_WIDTH))
                        .on_press(Message::RemoveProduct(index)),
                    text(product.id.map_or("None".to_string(), |id| id.to_string()))
                        .width(Length::Fixed(ID_WIDTH)),
                    text_input("EAN", &product.ean.clone().unwrap_or_default())
                        .width(Length::Fixed(EAN_WIDTH))
                        .on_input(move |value| Message::EanChange(index, value)),
                    row![
                        text_input("Nome", &product.name)
                            .on_input(move |value| Message::NameChange(index, value)),
                        button("BUSCAR").on_press(Message::SearchProduct(index))
                    ]
                    .width(NAME_WIDTH),
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

        column![
            header,
            scrollable(list_products.spacing(8)).height(Length::Fill)
        ]
        .into()
    }

    fn search(&self) -> Element<'_, Message> {
        let mut products = column![];

        for product in &self.search_products {
            products = products
                .push(
                    button(
                        row![
                            text(product.id.to_string()).width(Length::Fixed(ID_WIDTH)),
                            text(product.ean.clone().unwrap_or_default())
                                .width(Length::Fixed(EAN_WIDTH)),
                            text(&product.name).width(NAME_WIDTH),
                            text(format_int_to_decimal(product.price_sale))
                                .width(Length::Fixed(PRICE_UNIT_WIDTH)),
                        ]
                        .spacing(16)
                        .align_y(Alignment::Center),
                    )
                    .on_press(Message::ProductSelected(
                        self.search_index.unwrap(),
                        product.clone(),
                    )),
                )
                .spacing(8);
        }

        column![
            button("FECHAR").on_press(Message::CloseSearch),
            text_input("DIGITE O NOME DO PRODUTO", &self.search_text)
                .on_input(Message::SearchTextChange)
                .width(Length::Fill),
            products
        ]
        .spacing(16)
        .into()
    }

    fn total(&self) -> String {
        let mut total = 0;
        for product in &self.products {
            let quantity = product.quantity.parse::<i64>().unwrap_or(0);
            let price_unit = product
                .price_unit
                .replace(",", ".")
                .parse::<f64>()
                .unwrap_or(0.0);
            total += (quantity as f64 * price_unit * 100.0) as i64;
        }
        format_int_to_decimal(total)
    }
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

impl ProductItem {
    fn to_product(&self) -> Product {
        let price_sale = self
            .price_sale
            .replace(",", ".")
            .parse::<f64>()
            .unwrap_or(0.0);
        let price_purchase = self
            .price_unit
            .replace(",", ".")
            .parse::<f64>()
            .unwrap_or(0.0);
        let quantity = self.quantity.parse::<i64>().unwrap_or(0);
        Product::new(
            self.id.unwrap_or(0),
            self.name.clone(),
            f64_to_i64(price_sale),
            f64_to_i64(price_purchase),
            quantity,
            self.ean.clone(),
            Local::now().naive_local(),
        )
    }

    fn from_product(product: &Product) -> Self {
        let price_sale = i64_to_f64(product.price_sale);
        let price_purchase = i64_to_f64(product.price_purchase);
        let percentual = if price_purchase == 0.0 {
            0.0
        } else {
            ((price_sale - price_purchase) / price_purchase) * 100.0
        };
        Self {
            id: Some(product.id),
            ean: product.ean.clone(),
            name: product.name.clone(),
            quantity: "1".to_string(),
            price_unit: format!("{:.2}", price_purchase).replace(".", ","),
            price_sale: format!("{:.2}", price_sale).replace(".", ","),
            percentual: format!("{:.2}", percentual).replace(".", ","),
            total: format_int_to_decimal(product.price_sale * product.quantity),
            total_sale: format_int_to_decimal(product.price_sale * product.quantity),
        }
    }
}

fn calculate_total(product: &ProductItem) -> String {
    let quantity = product.quantity.parse::<f64>().unwrap_or(0.0);
    let price_unit = product
        .price_unit
        .replace(",", ".")
        .parse::<f64>()
        .unwrap_or(0.0);
    let total = f64_to_i64(quantity * price_unit);
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
    let total_sale = f64_to_i64(quantity * price_sale);
    format_int_to_decimal(total_sale)
}
