mod config;
mod entities;
mod helpers;
mod repositories;
mod screens;
mod services;

use crate::repositories::product_purchase_repository::ProductPurchaseRepository;
use crate::repositories::product_repository::ProductRepository;
use crate::services::product_purchase_service::ProductPurchaseService;
use iced::keyboard::{on_key_press, Key};
use iced::widget::{button, column, container, horizontal_rule, row};
use iced::{Element, Length, Subscription, Task, Theme};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;
use std::sync::Arc;
use iced::keyboard::key::Named;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> iced::Result {
    let pool = SqlitePool::connect("sqlite:database.db").await.unwrap();
    MIGRATOR.run(&pool).await.unwrap();
    let product_repository = Arc::new(ProductRepository::new(pool.clone()));
    let product_purchase_repository = Arc::new(ProductPurchaseRepository::new(pool.clone()));
    let product_service = Arc::new(ProductPurchaseService::new(
        product_purchase_repository,
        product_repository,
    ));

    iced::application("Teste", State::update, State::view)
        .subscription(State::subscription)
        .theme(State::theme)
        .centered()
        .run_with(|| State::new(product_service))
}

#[derive(Debug, Clone)]
enum Message {
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
}

impl State {
    fn new(product_purchase_service: Arc<ProductPurchaseService>) -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::AddPurchase(screens::add_purchase::State::new(
                    product_purchase_service.clone(),
                )),
                product_purchase_service,
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
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNightLight
    }

    fn subscription(&self) -> Subscription<Message> {
        on_key_press(|key, _| {
            if key == Key::Named(Named::F1) {

            }
            println!("{:?}", key);
            None
        })
    }
}
