use crate::{
    message::Message,
    model::{Model, RunningState},
};

pub fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Quit => model.running_state = RunningState::Done,
    }
}
