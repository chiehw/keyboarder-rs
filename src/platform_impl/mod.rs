#[cfg(target_os = "linux")]
#[path = "x11/mod.rs"]
mod platform;

pub use self::platform::*;
