mod config;
mod entities;
mod helpers;
mod repositories;
mod screens;
mod services;

use crate::services::product_purchase_service::ProductPurchaseService;
use crate::services::product_service::ProductService;
use iced::keyboard::key::Named;
use iced::keyboard::{on_key_press, Key};
use iced::widget::{button, column, container, horizontal_rule, row};
use iced::{Element, Length, Subscription, Task, Theme};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;
use std::sync::Arc;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> iced::Result {
    let pool = SqlitePool::connect("sqlite:database.db").await.unwrap();
    MIGRATOR.run(&pool).await.unwrap();
    let product_service = Arc::new(ProductService::new(pool.clone()));
    let product_purchase_service = Arc::new(ProductPurchaseService::new(pool.clone()));

    iced::application("Teste", State::update, State::view)
        .subscription(State::subscription)
        .theme(State::theme)
        .centered()
        .run_with(|| State::new(product_purchase_service, product_service))
}

#[derive(Debug, Clone)]
enum Message {
    NavigateToHome,
    NavigateToAddPurchase,
    Home(screens::home::Message),
    AddPurchase(screens::add_purchase::Message),
}

#[derive(Debug)]
enum Screen {
    Home(screens::home::State),
    AddPurchase(screens::add_purchase::State),
}

#[derive(Debug)]
struct State {
    screen: Screen,
    product_purchase_service: Arc<ProductPurchaseService>,
    product_service: Arc<ProductService>,
}

impl State {
    fn new(
        product_purchase_service: Arc<ProductPurchaseService>,
        product_service: Arc<ProductService>,
    ) -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::AddPurchase(screens::add_purchase::State::new(
                    product_purchase_service.clone(),
                    product_service.clone(),
                )),
                product_purchase_service,
                product_service,
            },
            Task::none(),
        )
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::Home(state) => state.view().map(Message::Home),
            Screen::AddPurchase(state) => state.view().map(Message::AddPurchase),
        };
        column![
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(16),
            horizontal_rule(1),
            row![
                button("(F1) INICIO"),
                button("(F2) PRODUTOS"),
                button("(F3) ESTOQUE"),
                button("(F4) FINANCEIRO"),
            ]
            .padding(16)
            .spacing(16)
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Home(message) => {
                if let Screen::Home(state) = &mut self.screen {
                    return state.update(message).map(Message::Home);
                }
            }
            Message::AddPurchase(message) => {
                if let Screen::AddPurchase(state) = &mut self.screen {
                    return state.update(message).map(Message::AddPurchase);
                }
            }

            Message::NavigateToHome => {
                self.screen = Screen::Home(screens::home::State::default());
            }
            Message::NavigateToAddPurchase => {
                self.screen = Screen::AddPurchase(screens::add_purchase::State::new(
                    self.product_purchase_service.clone(),
                    self.product_service.clone(),
                ));
            }
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightLight
    }

    fn subscription(&self) -> Subscription<Message> {
        on_key_press(|key, _| {
            match key {
                Key::Named(Named::F1) => {
                    return Some(Message::NavigateToHome);
                }
                Key::Named(Named::F2) => {
                    return Some(Message::NavigateToAddPurchase);
                }
                Key::Named(Named::F3) => {}
                Key::Named(Named::F4) => {}
                _ => return None,
            }

            println!("{:?}", key);
            None
        })
    }
}
