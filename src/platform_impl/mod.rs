#[cfg(target_os = "linux")]
#[path = "x11/mod.rs"]
mod platform;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

pub use self::platform::*;
