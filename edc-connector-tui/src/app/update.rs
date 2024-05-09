use super::{model::RunningState, msg::Message, App};

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => todo!(),
            Message::Decrement => todo!(),
            Message::Reset => todo!(),
            Message::Quit => self.model.running_state = RunningState::Done,
        }
    }
}
