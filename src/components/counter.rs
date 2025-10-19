use iced::widget::{button, column, text};
use iced::{Center, Element, Task};

#[derive(Default)]
pub struct View {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

pub enum Action {
    None,
    Run(Task<Message>),
}

impl View {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
        Action::None
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
