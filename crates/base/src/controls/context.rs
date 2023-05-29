use hashbrown::HashSet;
use notan::prelude::Plugin;

use super::{keyboard, Control};

pub struct ControlsContext {
    pub keyboard: keyboard::KeyMap,
    // #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
    // pub controller: gamepad::ButtonMap,
    pub touchscreen: TouchscreenContext,
}

#[derive(Default)]
pub struct TouchscreenContext {
    pub pressed: HashSet<Control>,
    pub down: HashSet<Control>,
}

impl Plugin for ControlsContext {}

impl Default for ControlsContext {
    fn default() -> Self {
        Self {
            keyboard: keyboard::default_key_map(),
            touchscreen: Default::default(),
            // #[cfg(all(not(target_arch = "wasm32"), feature = "gamepad"))]
            // controller: gamepad::default_button_map(),
        }
    }
}

impl TouchscreenContext {
    pub fn update(&mut self) {
        for control in self.pressed.drain() {
            self.down.insert(control);
        }
    }

    pub fn remove(&mut self, control: Control) {
        self.pressed.remove(&control);
        self.down.remove(&control);
    }
}
