pub use anyhow::{anyhow, Result};
pub use log;
pub use std::os::raw::c_int;

pub const TRUE: c_int = 1;
pub const FALSE: c_int = 0;

pub type KeyCode = u32;
pub type KeySym = u32;
