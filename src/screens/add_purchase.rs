use iced::widget::column;
use iced::{Element, Task};

#[derive(Debug)]
pub struct State {
    search_bar: String,
}

impl Default for State {
    fn default() -> Self {
        State {
            search_bar: "".to_string(),
        }
    }
}

impl State {
    pub fn view(&self) -> Element<Message> {
        column![].spacing(16).into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnSearchBarChange(value) => {
                self.search_bar = value;
            }
        }

        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    OnSearchBarChange(String),
}
