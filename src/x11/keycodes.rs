use std::{
    collections::HashMap,
    ffi::{CStr, OsStr},
    os::unix::prelude::OsStrExt,
};

use xkbcommon::xkb;

use super::{keyboard::Key, PhysKeyCode};
use crate::common::*;

macro_rules! decl_keycodes {
    ($($key:ident, $code:literal),*) => {
        #[allow(dead_code)]
        pub fn scancode_from_key(key:Key) -> u32 {
            match key {
                $(
                    Key::$key => $code,
                )*
            }
        }

        // #[allow(dead_code)]
        // pub fn key_from_scancode(scancode: u32) -> Key {
        //     match scancode {
        //         $(
        //             $code => Key::$key,
        //         )*
        //     }
        // }
    };
}

#[rustfmt::skip]
decl_keycodes!(
    AltLeft, 64,
    AltRight, 108,
    Backspace, 22,
    CapsLock, 66,
    ControlLeft, 37,
    ControlRight, 105,
    Delete, 119,
    DownArrow, 116,
    End, 115,
    Escape, 9,
    F1, 67,
    F10, 76,
    F11, 95,
    F12, 96,
    F2, 68,
    F3, 69,
    F4, 70,
    F5, 71,
    F6, 72,
    F7, 73,
    F8, 74,
    F9, 75,
    Home, 110,
    LeftArrow, 113,
    MetaLeft, 133,
    PageDown, 117,
    PageUp, 112,
    Return, 36,
    RightArrow, 114,
    ShiftLeft, 50,
    ShiftRight, 62,
    Space, 65,
    Tab, 23,
    UpArrow, 111,
    PrintScreen, 107,
    ScrollLock, 78,
    Pause, 127,
    NumLock, 77,
    BackQuote, 49,
    Num1, 10,
    Num2, 11,
    Num3, 12,
    Num4, 13,
    Num5, 14,
    Num6, 15,
    Num7, 16,
    Num8, 17,
    Num9, 18,
    Num0, 19,
    Minus, 20,
    Equal, 21,
    KeyQ, 24,
    KeyW, 25,
    KeyE, 26,
    KeyR, 27,
    KeyT, 28,
    KeyY, 29,
    KeyU, 30,
    KeyI, 31,
    KeyO, 32,
    KeyP, 33,
    LeftBracket, 34,
    RightBracket, 35,
    KeyA, 38,
    KeyS, 39,
    KeyD, 40,
    KeyF, 41,
    KeyG, 42,
    KeyH, 43,
    KeyJ, 44,
    KeyK, 45,
    KeyL, 46,
    SemiColon, 47,
    Quote, 48,
    BackSlash, 51,
    IntlBackslash, 94,
    KeyZ, 52,
    KeyX, 53,
    KeyC, 54,
    KeyV, 55,
    KeyB, 56,
    KeyN, 57,
    KeyM, 58,
    Comma, 59,
    Dot, 60,
    Slash, 61,
    Insert, 118,
    KpDecimal, 91,
    KpReturn, 104,
    KpMinus, 82,
    KpPlus, 86,
    KpMultiply, 63,
    KpDivide, 106,
    Kp0, 90,
    Kp1, 87,
    Kp2, 88,
    Kp3, 89,
    Kp4, 83,
    Kp5, 84,
    Kp6, 85,
    Kp7, 79,
    Kp8, 80,
    Kp9, 81,
    MetaRight, 134,
    Apps, 135
);

