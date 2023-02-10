use crate::event::{CodeState, State};
use rdev::Key;

pub enum Action {
    Sync(CodeState),
    Reverse(CodeState),
}
