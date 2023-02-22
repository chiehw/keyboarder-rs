use std::rc::Rc;

use crate::connection::ConnectionOps;

pub struct WinConnection {}

impl WinConnection {
    pub fn create_new() -> anyhow::Result<WinConnection> {
        anyhow::Ok(Self {})
    }
}

impl ConnectionOps for WinConnection {}
