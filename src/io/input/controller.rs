use gilrs::Button;
use gilrs::Gilrs;
use parking_lot::Mutex;
use parking_lot::RwLock;
use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;

lazy_static::lazy_static! {
    pub static ref CONTROLLER_CONTROLS: RwLock<HashMap<Control, Button>> = RwLock::new(default());
}

lazy_static::lazy_static! {
    static ref GILRS: Mutex<Gilrs> = Mutex::new(Gilrs::new().expect("Could not load GilRs!"));
    pub static ref BUTTON_MAP: RwLock<HashMap<Button, bool>> = RwLock::new(HashMap::new());
    static ref BUTTONS: HashSet<Button> = buttons();
}

pub fn pressed(control: &Control) -> bool {
    // if let Some(button) = CONTROLLER_CONTROLS.read().get(&control) {
    //     if let Some(val) = self::controller::BUTTON_MAP.read().get(button) {
    //         if val.eq(&true) {
    //             return true;
    //         }
    //     }
    // }
    return false;
}

pub fn default() -> HashMap<Control, Button> {
    let mut controls = HashMap::new();
    controls.insert(Control::A, Button::South);
    controls.insert(Control::B, Button::West);
    controls.insert(Control::Up, Button::DPadUp);
    controls.insert(Control::Down, Button::DPadDown);
    controls.insert(Control::Left, Button::DPadLeft);
    controls.insert(Control::Right, Button::DPadRight);
    controls.insert(Control::Start, Button::Start);
    controls.insert(Control::Select, Button::Select);
    controls
}

pub fn update_active_controls() {
    let mut gilrs = GILRS.lock();

    let mut active_gamepad = None;

    while let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
        active_gamepad = Some(id);
    }

    if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
        let mut map = BUTTON_MAP.write();
        for button in &*BUTTONS {
            if gamepad.is_pressed(*button) {
                map.insert(*button, true);
            } else {
                map.insert(*button, false);
            }
        }
    }
    

}

fn buttons() -> HashSet<Button> {
    let mut buttons = HashSet::new();
    buttons.insert(Button::South);
    buttons.insert(Button::West);
    buttons.insert(Button::DPadUp);
    buttons.insert(Button::DPadDown);
    buttons.insert(Button::DPadLeft);
    buttons.insert(Button::DPadRight);
    buttons.insert(Button::Start);
    buttons.insert(Button::Select);
    let mut map = BUTTON_MAP.write();
    for button in &buttons {
        map.insert(*button, false);
    }
    return buttons;
}