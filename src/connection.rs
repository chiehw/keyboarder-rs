use crate::platform_impl::{Connection, Simulator};
use crate::simulate::Simulate;
use crate::types::ServerMode;

use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    pub static CONN: RefCell<Option<Rc<Connection>>> = RefCell::new(None);
}

pub trait ConnectionOps {
    fn init() -> anyhow::Result<Rc<Connection>> {
        let conn = Rc::new(Connection::create_new()?);
        CONN.with(|m| *m.borrow_mut() = Some(Rc::clone(&conn)));

        Ok(conn)
    }

    fn get() -> Option<Rc<Connection>> {
        let mut res = None;
        CONN.with(|m| {
            if let Some(mux) = &(*m.borrow()) {
                res = Some(Rc::clone(mux));
            }
        });
        res
    }

    fn with_simulator(mode: ServerMode) -> anyhow::Result<Rc<Connection>> {
        let conn = Rc::new(Connection::create_new()?);
        CONN.with(|m| *m.borrow_mut() = Some(Rc::clone(&conn)));

        let mut simulator = Simulator::new(&conn);
        simulator.mode.replace(mode);
        simulator.release_modifiers()?;
        conn.simulator.borrow_mut().replace(simulator);

        Ok(conn)
    }

    fn process_event() {}
}
