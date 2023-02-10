use crate::event::CodeState;

pub enum Action {
    Sync(CodeState),
    Reverse(CodeState),
}
