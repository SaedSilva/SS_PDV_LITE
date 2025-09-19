use crate::entities::product::Product;
use crate::helpers::format_int_to_decimal;
use crate::services::product_service::ProductService;
use iced::widget::{column, horizontal_rule, row, text};
use iced::{Alignment, Element, Length, Task};
use std::fmt::Display;
use std::sync::Arc;
use crate::components::combo_box;
use crate::components::combo_box::combo_box;

#[derive(Debug)]
pub struct State {
    product_service: Arc<ProductService>,
    search_bar: String,
    search_bar_products: combo_box::State<ProductItem>,
    products: Vec<ProductItem>,
}

impl State {
    pub fn new(product_service: Arc<ProductService>) -> Self {
        State {
            product_service,
            search_bar: String::new(),
            search_bar_products: combo_box::State::default(),
            products: vec![],
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            combo_box(
                &self.search_bar_products,
                "DIGITE O NOME DO PRODUTO",
                None,
                Message::SelectProduct
            )
            .on_input(Message::OnSearchBarChange),
            row![
                self.product_list(),
                column![
                    text("TOTAL").size(64),
                    text(format_int_to_decimal(self.total_value())).size(64)
                ]
                .align_x(Alignment::Center)
                .width(Length::Fill)
            ]
        ]
        .spacing(16)
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnSearchBarChange(value) => {
                self.search_bar = value.clone();
                if value.is_empty() {
                    self.search_bar_products.change_options(vec![]);
                } else {
                    let product_service = self.product_service.clone();
                    let search_text = self.search_bar.clone();
                    return Task::perform(
                        async move { product_service.search_products_by_name(&search_text).await },
                        Message::SearchedProducts,
                    );
                }
            }
            Message::SelectProduct(product) => {
                self.products.push(product);
                self.search_bar = String::new();
                self.search_bar_products.clear_text();
                return Task::done(Message::OnSearchBarChange(String::new()));
            }
            Message::SearchedProducts(products) => {
                let items: Vec<ProductItem> = products
                    .into_iter()
                    .map(ProductItem::from_product)
                    .collect();
                self.search_bar_products.change_options(items);
            }
        }

        Task::none()
    }

    fn product_list(&self) -> Element<Message> {
        let mut list = column![
            row![
                text("PRODUTO\nPRECO UNIT.").width(Length::Fill),
                text("QUANTIDADE"),
                text("TOTAL"),
            ]
            .spacing(16)
        ];

        for product in &self.products {
            list = list
                .push(
                    row![
                        column![
                            text(&product.name),
                            text(format_int_to_decimal(product.value)),
                        ]
                        .width(Length::Fill),
                        text(product.quantity),
                        text(format_int_to_decimal(product.total_value())),
                    ]
                    .spacing(16),
                )
                .push(horizontal_rule(1));
        }

        list.into()
    }

    fn total_value(&self) -> i64 {
        self.products.iter().map(|x| x.total_value()).sum()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    OnSearchBarChange(String),
    SelectProduct(ProductItem),
    SearchedProducts(Vec<Product>),
}

#[derive(Debug, Clone)]
struct ProductItem {
    name: String,
    quantity: i64,
    value: i64,
}

impl ProductItem {
    fn total_value(&self) -> i64 {
        (self.quantity * self.value)
    }

    fn from_product(product: Product) -> Self {
        ProductItem {
            name: product.name,
            quantity: 1,
            value: product.price_sale,
        }
    }
}

impl Default for ProductItem {
    fn default() -> Self {
        ProductItem {
            name: "Produto".to_string(),
            quantity: 2,
            value: 1050,
        }
    }
}

impl Display for ProductItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, format_int_to_decimal(self.value))
    }
}
