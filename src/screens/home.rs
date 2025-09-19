use crate::components::combo_box;
use crate::components::combo_box::combo_box;
use crate::entities::product::Product;
use crate::helpers::format_int_to_decimal;
use crate::services::product_sale_service::ProductSaleService;
use crate::services::product_service::ProductService;
use iced::widget::{button, column, horizontal_rule, row, text, vertical_space};
use iced::{Alignment, Element, Length, Task};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug)]
pub struct State {
    product_service: Arc<ProductService>,
    product_sale_service: Arc<ProductSaleService>,
    search_bar: String,
    search_bar_products: combo_box::State<ProductItem>,
    products: Vec<ProductItem>,
}

impl State {
    pub fn new(
        product_service: Arc<ProductService>,
        product_sale_service: Arc<ProductSaleService>,
    ) -> Self {
        State {
            product_service,
            product_sale_service,
            search_bar: String::new(),
            search_bar_products: combo_box::State::default(),
            products: vec![],
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
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
                    text(format_int_to_decimal(self.total_value())).size(64),
                    vertical_space(),
                    button(text("FINALIZAR VENDA").align_x(Alignment::Center))
                        .padding(16)
                        .width(Length::Fill)
                        .on_press(Message::FinishSale),
                ]
                .align_x(Alignment::Center)
                .width(Length::FillPortion(1))
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
            Message::RemoveProduct(index) => {
                if index < self.products.len() {
                    self.products.remove(index);
                }
            }
            Message::DecreaseProductQuantity(index) => {
                if index < self.products.len() {
                    let product = &mut self.products[index];
                    if product.quantity > 1 {
                        product.quantity -= 1;
                    }
                }
            }
            Message::IncreaseProductQuantity(index) => {
                if index < self.products.len() {
                    let product = &mut self.products[index];
                    if product.quantity < product.stock {
                        product.quantity += 1;
                    }
                }
            }
            Message::FinishSale => {
                if self.products.is_empty() {
                    return Task::none();
                }

                let sale_products: Vec<Product> =
                    self.products.iter().map(|p| p.to_product()).collect();
                let product_sale_service = self.product_sale_service.clone();
                return Task::perform(
                    async move {
                        match product_sale_service.add_sale(sale_products).await {
                            Ok(_) => Message::OnSaleFinished(Some(
                                "Venda finalizada com sucesso!".to_string(),
                            )),
                            Err(e) => Message::OnSaleFinished(Some(format!(
                                "Erro ao finalizar venda: {}",
                                e
                            ))),
                        }
                    },
                    |msg| msg,
                );
            }
            Message::OnSaleFinished(value) => {
                self.products.clear();
                if let Some(msg) = value {
                    println!("{}", msg);
                }
            }
        }

        Task::none()
    }

    fn product_list(&self) -> Element<'_, Message> {
        let quantity_width = 100;
        let total_width = 100;
        let action_width = 120;
        let mut list = column![
            row![
                text("PRODUTO\nPRECO UNIT.").width(Length::FillPortion(4)),
                text("QUANTIDADE").width(quantity_width),
                text("TOTAL").width(total_width),
                text("AÇÕES").width(action_width),
            ]
            .spacing(16),
            horizontal_rule(2),
        ];

        for (index, product) in self.products.iter().enumerate() {
            let button_more = if product.quantity < product.stock {
                button("+").on_press(Message::IncreaseProductQuantity(index))
            } else {
                button("+")
            };
            let button_less = if product.quantity > 1 {
                button("-").on_press(Message::DecreaseProductQuantity(index))
            } else {
                button("-")
            };
            let buttons_row = row![
                button("X").on_press(Message::RemoveProduct(index)),
                button_less,
                button_more,
            ]
            .spacing(4);

            list = list
                .push(
                    row![
                        column![
                            text(&product.name),
                            text(format_int_to_decimal(product.value)),
                        ]
                        .width(Length::FillPortion(4)),
                        text(product.quantity).width(quantity_width),
                        text(format_int_to_decimal(product.total_value())).width(total_width),
                        buttons_row.width(action_width),
                    ]
                    .spacing(16)
                    .align_y(Alignment::Center),
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
    RemoveProduct(usize),
    DecreaseProductQuantity(usize),
    IncreaseProductQuantity(usize),
    FinishSale,
    OnSaleFinished(Option<String>),
}

#[derive(Debug, Clone)]
pub(crate) struct ProductItem {
    id: i64,
    ean: Option<String>,
    name: String,
    quantity: i64,
    value: i64,
    value_purchase: i64,
    stock: i64,
}

impl ProductItem {
    fn total_value(&self) -> i64 {
        (self.quantity * self.value)
    }

    fn from_product(product: Product) -> Self {
        ProductItem {
            id: product.id,
            ean: product.ean,
            name: product.name,
            quantity: 1,
            value: product.price_sale,
            value_purchase: product.price_purchase,
            stock: product.quantity,
        }
    }

    fn to_product(&self) -> Product {
        Product::new(
            self.id,
            self.name.clone(),
            self.value,
            self.value_purchase,
            self.quantity,
            self.ean.clone(),
            chrono::Local::now().naive_local(),
        )
    }
}

impl Default for ProductItem {
    fn default() -> Self {
        ProductItem {
            id: 0,
            ean: None,
            name: "Produto".to_string(),
            quantity: 2,
            value: 1050,
            value_purchase: 800,
            stock: 10,
        }
    }
}

impl Display for ProductItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, format_int_to_decimal(self.value))
    }
}
