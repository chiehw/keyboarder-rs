use crate::types::{KeyEvent, PhysKeyCode};
use filedescriptor::FileDescriptor;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

lazy_static::lazy_static! {
    pub static ref  SENDER: Arc<Mutex<Option<FileDescriptor>>> = Default::default();
}

pub trait Simulate {
    fn spawn_server() -> anyhow::Result<JoinHandle<()>>;

    fn event_to_server(key_event: &KeyEvent) -> anyhow::Result<()>;

    fn simulate_keycode(&mut self, keycode: u32, press: bool);

    fn simulate_keysym(&mut self, keysym: u32, press: bool);

    fn simulate_char_without_modifiers(&mut self, chr: char);

    fn simulate_phys(&mut self, phys: PhysKeyCode, press: bool);

    fn simulate_key_event(&mut self, key_event: &KeyEvent);
}
