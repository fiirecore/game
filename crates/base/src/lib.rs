pub extern crate bevy_egui;
pub extern crate firecore_text as text;

use bevy::prelude::PluginGroup;
pub use bevy::utils::{hashbrown::hash_map::DefaultHashBuilder as RandomState, HashMap, HashSet};

pub mod audio;
pub mod controls;
pub mod gui;

pub struct FirecorePlugins;

impl PluginGroup for FirecorePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(bevy_egui::EguiPlugin)
            .add(controls::ControlsPlugin)
            .add(audio::AudioPlugin)
    }
}
