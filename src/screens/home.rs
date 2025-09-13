use iced::Element;
use iced::widget::container;

#[derive(Debug, Default)]
pub struct State {
    search_bar: String,
}

impl State {
    pub fn view(&self) -> Element<Message> {
        container("").into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {

}