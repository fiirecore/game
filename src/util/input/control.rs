use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use macroquad::prelude::KeyCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Control {

    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    //Escape,

}

impl Control {

    pub fn default_map() -> HashMap<Control, HashSet<KeyCode>> {
        let mut controls = HashMap::new();
        controls.insert(Control::A, set_of(&[KeyCode::X]));
        controls.insert(Control::B, set_of(&[KeyCode::Z]));
        controls.insert(Control::Up, set_of(&[KeyCode::Up]));
        controls.insert(Control::Down, set_of(&[KeyCode::Down]));
        controls.insert(Control::Left, set_of(&[KeyCode::Left]));
        controls.insert(Control::Right, set_of(&[KeyCode::Right]));
        controls.insert(Control::Start, set_of(&[KeyCode::A]));
        controls.insert(Control::Select, set_of(&[KeyCode::S]));
        controls
    }

}

fn set_of(codes: &[KeyCode]) -> HashSet<KeyCode> {
    let mut set = HashSet::new();
    for code in codes {
        set.insert(*code);
    }    
    return set;
}