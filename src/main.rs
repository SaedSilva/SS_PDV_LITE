mod config;
mod helpers;
mod repositories;
mod screens;
mod services;

use iced::keyboard::on_key_press;
use iced::widget::{button, column, container, row};
use iced::{Element, Length, Subscription, Task, Theme};

fn main() -> iced::Result {
    iced::application("Teste", State::update, State::view)
        .subscription(State::subscription)
        .theme(State::theme)
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
            container(content).width(Length::Fill).height(Length::Fill),
            row![
                button("(F1) INICIO"),
                button("(F2) PRODUTOS"),
                button("(F3) ESTOQUE"),
                button("(F4) FINANCEIRO"),
            ]
            .spacing(16)
        ]
        .padding(16)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
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
