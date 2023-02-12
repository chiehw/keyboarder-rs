pub use anyhow::{anyhow, ensure, Result};
pub use log;
pub use std::os::raw::c_int;
use std::ptr::null;
use x11::xlib;

pub const TRUE: c_int = 1;
pub const FALSE: c_int = 0;

pub type KeyCode = u32;
pub type KeySym = u32;
