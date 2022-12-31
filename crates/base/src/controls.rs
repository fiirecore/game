use bevy::{
    input::{Input, InputSystem},
    prelude::{App, CoreStage, KeyCode, Plugin, Res, ResMut, Resource, IntoSystemDescriptor},
    utils::HashMap,
};
use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize, Enum)]
pub enum Control {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}

impl Control {
    pub fn name(&self) -> &str {
        match self {
            Control::A => "A",
            Control::B => "B",
            Control::Up => "Up",
            Control::Down => "Down",
            Control::Left => "Left",
            Control::Right => "Right",
            Control::Start => "Start",
            Control::Select => "Select",
        }
    }
}

#[derive(Resource)]
pub struct ControlMappings {
    pub keys: HashMap<KeyCode, Control>,
}

impl Default for ControlMappings {
    fn default() -> Self {
        Self {
            keys: {
                let mut map = HashMap::with_capacity(8);
                map.insert(KeyCode::X, Control::A);
                map.insert(KeyCode::Z, Control::B);
                map.insert(KeyCode::Up, Control::Up);
                map.insert(KeyCode::Down, Control::Down);
                map.insert(KeyCode::Left, Control::Left);
                map.insert(KeyCode::Right, Control::Right);
                map.insert(KeyCode::A, Control::Start);
                map.insert(KeyCode::S, Control::Select);
                map
            },
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Input<Control>>()
            .init_resource::<ControlMappings>()
            .add_system_to_stage(CoreStage::PreUpdate, update.after(InputSystem));
    }
}

fn update(
    mut controls: ResMut<Input<Control>>,
    mappings: Res<ControlMappings>,
    keyboard: Res<Input<KeyCode>>,
) {
    controls.clear();
    for (key, control) in mappings.keys.iter() {
        if keyboard.just_pressed(*key) {
            controls.press(*control);
        }
        if keyboard.just_released(*key) {
            controls.release(*control);
        }
    }
}
