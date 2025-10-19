use crate::components::counter;
use iced::Task;
use iced::widget::row;

#[derive(Default)]
pub struct View {
    counter_1: counter::View,
    counter_2: counter::View,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CounterOneMessage(counter::Message),
    CounterTwoMessage(counter::Message),
}

pub enum Action {
    None,
    Run(Task<Message>),
}

impl View {
    pub fn new() -> Self {
        let counter_1 = counter::View::new();
        let counter_2 = counter::View::new();
        Self {
            counter_1,
            counter_2,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::CounterOneMessage(msg) => match self.counter_1.update(msg) {
                counter::Action::None => Action::None,
                counter::Action::Run(task) => Action::Run(task.map(Message::CounterOneMessage)),
            },
            Message::CounterTwoMessage(msg) => match self.counter_2.update(msg) {
                counter::Action::None => Action::None,
                counter::Action::Run(task) => Action::Run(task.map(Message::CounterTwoMessage)),
            },
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        row![
            self.counter_1.view().map(Message::CounterOneMessage),
            self.counter_2.view().map(Message::CounterTwoMessage)
        ]
        .into()
    }
}
