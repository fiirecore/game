use notan::prelude::Plugins;

use super::{Control, context::ControlsContext};

pub fn pressed(plugins: &Plugins, control: Control) -> bool {
    plugins
        .get::<ControlsContext>()
        .map(|c| c.touchscreen.pressed.contains(&control))
        .unwrap_or_default()
}

pub fn down(plugins: &Plugins, control: Control) -> bool {
    plugins
        .get::<ControlsContext>()
        .map(|c| c.touchscreen.down.contains(&control))
        .unwrap_or_default()
}
