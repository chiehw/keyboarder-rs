use crate::connection::ConnectionOps;
use crate::platform_impl::platform::simulator::WinSimulator;
use std::cell::RefCell;

pub struct WinConnection {
    pub simulator: RefCell<Option<WinSimulator>>,
}

impl WinConnection {
    pub fn create_new() -> anyhow::Result<WinConnection> {
        anyhow::Ok(Self {
            simulator: RefCell::new(None),
        })
    }
}

impl ConnectionOps for WinConnection {}
