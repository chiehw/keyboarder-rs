use crate::types::{KeyEvent, PhysKeyCode, SimulateEvent};
use crate::{connection, platform_impl::Connection};
use anyhow::Context;
use connection::ConnectionOps;
// use crossbeam::{channel, select};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::mpsc;
use std::thread::JoinHandle;

thread_local! {
    static SENDER: RefCell<Option<mpsc::Sender<SimulateEvent>>> = RefCell::new(None);
    pub static RECIVER: RefCell<Option<mpsc::Receiver<SimulateEvent>>> = RefCell::new(None);
}

pub trait Simulate {
    fn spawn_server() -> anyhow::Result<JoinHandle<()>> {
        let (sender, reciver) = mpsc::channel();
        SENDER.with(|m| *m.borrow_mut() = Some(sender));

        Ok(std::thread::spawn(move || {
            let conn = Connection::with_simulator()
                .context("Failed to init Connection")
                .unwrap();

            loop {
                if let Ok(sim_event) = reciver.recv() {
                    conn.process_simulate_event(sim_event)
                        .map_err(|err| log::error!("simulate error: {:?}", err))
                        .ok();
                }
            }
        }))
    }

    fn simulate_event_to_server(sim_event: SimulateEvent) -> anyhow::Result<()> {
        SENDER.with(|sender| {
            if let Some(sender) = &(*sender.borrow()) {
                sender
                    .send(sim_event)
                    .map_err(|err| log::error!("simulate error: {:?}", err))
                    .ok();
            }
        });

        Ok(())
    }

    fn simulate_event(&mut self, sim_event: SimulateEvent);

    fn simulate_keycode(&mut self, keycode: u32, press: bool);

    fn simulate_keysym(&mut self, keysym: u32, press: bool);

    fn simulate_char_without_modifiers(&mut self, chr: char);

    fn simulate_phys(&mut self, phys: PhysKeyCode, press: bool);

    fn simulate_key_event(&mut self, key_event: &KeyEvent);
}
