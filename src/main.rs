mod components;
use components::two_counters;
use iced::Task;

#[derive(Debug, Clone)]
enum Message {
    TwoCounters(two_counters::Message),
}

enum View {
    TwoCounters(two_counters::View),
}

impl Default for View {
    fn default() -> Self {
        let tc = two_counters::View::new();
        View::TwoCounters(tc)
    }
}

#[derive(Default)]
struct App {
    view: View,
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::TwoCounters(tc_msg) => match &mut self.view {
                View::TwoCounters(tc) => match tc.update(tc_msg) {
                    two_counters::Action::Run(task) => return task.map(Message::TwoCounters),
                    two_counters::Action::None => Task::none(),
                },
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        match &self.view {
            View::TwoCounters(two_counters) => two_counters.view().map(Message::TwoCounters),
        }
    }
}

pub fn main() -> iced::Result {
    iced::run("A cool counter", App::update, App::view)
}
