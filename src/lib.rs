use types::{KeyEvent, PhysKeyCode};

pub mod connection;
pub mod platform_impl;
pub mod types;
pub mod utils;

pub trait Simulate {
    fn simulate_keycode(&mut self, keycode: u32, press: bool);

    fn simulate_keysym(&mut self, keysym: u32, press: bool);

    fn simulate_char_without_modifiers(&mut self, chr: char);

    fn simulate_phys(&mut self, phys: PhysKeyCode, press: bool);

    fn simulate_key_event(&mut self, key_event: &KeyEvent);
}
