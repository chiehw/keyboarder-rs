// use std::ptr::null_mut;
// use keyboarder::platform_impl::Keyboard;
// use winapi::um::winuser::{GetForegroundWindow, GetKeyboardLayout, GetWindowThreadProcessId};

// fn main() {
//     env_logger::init();
//     std::env::set_var("DISPLAY", ":0");
//     std::env::set_var("RUST_LOG", "trace");

//     // WListener::new().run();

//     unsafe {
//         let current_window_thread_id = GetWindowThreadProcessId(GetForegroundWindow(), null_mut());
//         let _hkl = GetKeyboardLayout(current_window_thread_id);
//     }

//     Keyboard::new().get_current_modifiers();
// }

fn main() {}
