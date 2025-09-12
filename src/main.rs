mod screens;
mod services;
mod repositories;
mod helpers;

use iced::widget::container;
use iced::{Element, Task, Theme};

fn main() -> iced::Result {
    iced::application("Teste", State::update, State::view)
        .theme(State::theme)
        .run()
}

#[derive(Debug)]
enum Message {}

#[derive(Debug, Default)]
struct State;

impl State {
    fn view(&self) -> Element<Message> {
        container("Teste").into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinLatte
    }
}
