use macroquad::prelude::KeyCode;
use serde::{Deserialize, Serialize};
use ahash::AHashMap as HashMap;

use crate::Control;
use crate::KeySet;
use crate::KeySetSerializable;

pub fn ser_map(map: HashMap<Control, KeySet>) -> HashMap<Control, KeySetSerializable> {
    map.into_iter().map(|(control, keys)| {
        (control, ser_set(keys))
    }).collect()
}

pub fn ser_set(set: KeySet) -> KeySetSerializable {
    set.into_iter().map(|key| KeySerializable::c(key)).collect()
}

pub fn normal_map(map: &HashMap<Control, KeySetSerializable>) -> HashMap<Control, KeySet> {
    map.iter().map(|(control, keys)| {
        (*control, normal_set(keys))
    }).collect()
}

pub fn normal_set(set: &KeySetSerializable) -> KeySet {
    set.iter().map(|ks| {
        ks.code
    }).collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Deserialize, Serialize)]
pub struct KeySerializable {

    #[serde(with = "KeyCodeDef")]
    pub code: KeyCode,
}

impl KeySerializable {

    pub fn c(code: KeyCode) -> Self {
        Self {
            code
        }
    }

}

#[serde(remote = "KeyCode")]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Deserialize, Serialize)]
enum KeyCodeDef {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}