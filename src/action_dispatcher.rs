use crate::common::*;
use x11::xlib::{Display, XCloseDisplay, XOpenDisplay};

pub struct ActionDispatcher {
    display: *mut Display,
}

impl ActionDispatcher {
    pub fn new() -> Result<Self> {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());
            if display.is_null() {
                return Err(anyhow!("Missing Display, Try `export Display=:0`"));
            }
            Ok(Self { display })
        }
    }
}

impl Drop for ActionDispatcher {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.display);
        }
    }
}
