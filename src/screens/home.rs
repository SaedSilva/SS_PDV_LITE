use crate::helpers::format_int_to_decimal;
use iced::widget::{column, horizontal_rule, row, text, text_input};
use iced::{Alignment, Element, Length, Task};

#[derive(Debug)]
pub struct State {
    search_bar: String,
    products: Vec<Product>,
}

impl Default for State {
    fn default() -> Self {
        State {
            search_bar: "".to_string(),
            products: vec![Product::default(), Product::default(), Product::default()],
        }
    }
}

impl State {
    pub fn view(&self) -> Element<Message> {
        column![
            text_input(
                "Digite o código de barras ou o nome do produto",
                &self.search_bar
            )
            .width(Length::Fill)
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
                self.search_bar = value;
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
}

#[derive(Debug)]
struct Product {
    name: String,
    quantity: i64,
    value: i64,
}

impl Product {
    fn total_value(&self) -> i64 {
        (self.quantity * self.value)
    }
}

impl Default for Product {
    fn default() -> Self {
        Product {
            name: "Produto".to_string(),
            quantity: 2,
            value: 1050,
        }
    }
}
