mod config;
mod helpers;
mod repositories;
mod screens;
mod services;
mod entities;

use iced::keyboard::on_key_press;
use iced::widget::{button, column, container, horizontal_rule, row};
use iced::{Element, Length, Subscription, Task, Theme};
use sqlx::migrate::Migrator;
use sqlx::SqlitePool;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> iced::Result {
    let pool = SqlitePool::connect("sqlite:database.db").await.unwrap();
    MIGRATOR.run(&pool).await.unwrap();

    iced::application("Teste", State::update, State::view)
        .subscription(State::subscription)
        .theme(State::theme)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    HomeMessage(screens::home::Message),
}

#[derive(Debug)]
enum Screen {
    Home(screens::home::State),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home(screens::home::State::default())
    }
}

#[derive(Debug, Default)]
struct State {
    screen: Screen,
}

impl State {
    fn view(&self) -> Element<Message> {
        let content = match &self.screen {
            Screen::Home(state) => state.view().map(Message::HomeMessage),
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
            Message::HomeMessage(message) => {
                if let Screen::Home(state) = &mut self.screen {
                    return state.update(message).map(Message::HomeMessage);
                }
            }
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }

    fn subscription(&self) -> Subscription<Message> {
        on_key_press(|key, _| {
            println!("{:?}", key);
            None
        })
    }
}
