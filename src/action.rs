use crate::event::CodeState;

pub enum Action {
    Simulate(CodeState),
    Reverse(CodeState),
}
