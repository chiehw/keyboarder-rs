use crate::types::{KeyEvent, PhysKeyCode, SimulateEvent};
use crate::{connection, platform_impl::Connection};
use anyhow::Context;
use connection::ConnectionOps;
use filedescriptor::{FileDescriptor, Pipe};
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

lazy_static::lazy_static! {
    pub static ref  SENDER: Arc<Mutex<Option<FileDescriptor>>> = Default::default();
}

pub trait Simulate {
    fn spawn_server() -> anyhow::Result<JoinHandle<()>> {
        let pipe = Pipe::new()?;

        let mut write_fd = pipe.write;
        let mut read_fd = pipe.read;

        write_fd.set_non_blocking(true)?;
        read_fd.set_non_blocking(true)?;

        SENDER.lock().unwrap().replace(write_fd);

        Ok({
            std::thread::spawn(move || {
                let conn = Connection::with_simulator()
                    .context("Failed to init Connection")
                    .unwrap();

                if let Err(err) = conn.run_message_loop(&mut read_fd) {
                    log::error!("Failed to process message: {:?}", err);
                };
            })
        })
    }

    fn event_to_server(key_event: &KeyEvent) -> anyhow::Result<()> {
        let mut binding = SENDER.lock().unwrap();
        let sender = binding.as_mut();

        if let Some(sender) = sender {
            let buf = key_event.to_u8_vec()?;
            let size = sender.write(&buf)?;
            if size != buf.len() {
                log::error!("Can't write key event");
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        Ok(())
    }

    fn simulate_event(&mut self, sim_event: SimulateEvent);

    fn simulate_keycode(&mut self, keycode: u32, press: bool);

    fn simulate_keysym(&mut self, keysym: u32, press: bool);

    fn simulate_char_without_modifiers(&mut self, chr: char);

    fn simulate_phys(&mut self, phys: PhysKeyCode, press: bool);

    fn simulate_key_event(&mut self, key_event: &KeyEvent);
}
