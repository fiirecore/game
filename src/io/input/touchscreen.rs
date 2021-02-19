use macroquad::prelude::Color;
use macroquad::prelude::MouseButton;
use macroquad::prelude::collections::storage::get;
use crate::io::data::configuration::Configuration;

use super::Control;

lazy_static::lazy_static! {
    pub static ref TOUCH_CONTROLS: TouchControls = TouchControls::default();
}

static BUTTON_SIZE: f32 = 20.0;

pub struct TouchControls {

    a: TouchButton,
    b: TouchButton,

    down: TouchButton,
    up: TouchButton,
    left: TouchButton,
    right: TouchButton,

}

impl TouchControls {

    pub fn render(&self) {
        self.a.draw();
        self.b.draw();
        self.down.draw();
        self.up.draw();
        self.left.draw();
        self.right.draw();
    }

    pub fn pressed(&self, control: &Control) -> bool {
        if let Some(config) = get::<Configuration>() {
            if config.touchscreen {
                if macroquad::input::is_mouse_button_pressed(MouseButton::Left) {
                    if self.a.mouse_over() {
                        if self.a.control.eq(control) {
                            return true;
                        }
                    }
                    if self.b.mouse_over() {
                        if self.b.control.eq(control) {
                            return true;
                        }
                    }
                    if self.up.mouse_over() {
                        if self.up.control.eq(control) {
                            return true;
                        }
                    }
                    if self.down.mouse_over() {
                        if self.down.control.eq(control) {
                            return true;
                        }
                    }
                    if self.left.mouse_over() {
                        if self.left.control.eq(control) {
                            return true;
                        }
                    }
                    if self.right.mouse_over() {
                        if self.right.control.eq(control) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn down(&self, control: &Control) -> bool {
        if let Some(config) = get::<Configuration>() {
            if config.touchscreen {
                if macroquad::input::is_mouse_button_down(MouseButton::Left) {
                    if self.a.mouse_over() {
                        if self.a.control.eq(control) {
                            return true;
                        }
                    }
                    if self.b.mouse_over() {
                        if self.b.control.eq(control) {
                            return true;
                        }
                    }
                    if self.up.mouse_over() {
                        if self.up.control.eq(control) {
                            return true;
                        }
                    }
                    if self.down.mouse_over() {
                        if self.down.control.eq(control) {
                            return true;
                        }
                    }
                    if self.left.mouse_over() {
                        if self.left.control.eq(control) {
                            return true;
                        }
                    }
                    if self.right.mouse_over() {
                        if self.right.control.eq(control) {
                            return true;
                        }
                    }
                }
            }
        }  
        return false;
    }

}

impl Default for TouchControls {
    fn default() -> Self {
        Self {
            a: TouchButton::new(20.0, 100.0, Control::A),
            b: TouchButton::new(10.0, 130.0, Control::B),
            down: TouchButton::new(190.0, 130.0, Control::Down),
            up: TouchButton::new(190.0, 90.0, Control::Up),
            left: TouchButton::new(170.0, 110.0, Control::Left),
            right: TouchButton::new(210.0, 110.0, Control::Right),
        }
    }
}

pub struct TouchButton {

    x: f32,
    y: f32,
    color: Color, // replace with texture
    control: Control,

}

impl TouchButton {

    pub fn new(x: f32, y: f32, control: Control) -> Self {
        Self {
            x,
            y,
            color: macroquad::prelude::RED,
            control,
        }
    }

    pub fn draw(&self) {
        if let Some(config) = get::<Configuration>() {
            if config.touchscreen {
                crate::util::graphics::draw_rect(self.color, self.x, self.y, BUTTON_SIZE as u32, BUTTON_SIZE as u32);
                crate::util::graphics::draw_text_left(1, &format!("{:?}", self.control), self.x + 1.0, self.y);
            }
        }
    }

    pub fn mouse_over(&self) -> bool {
        let pos = macroquad::input::mouse_position();
        let pos_x = pos.0 / crate::SCALE;
        let pos_y = pos.1 / crate::SCALE;
        if self.x <= pos_x && self.x + BUTTON_SIZE > pos_x {
            if self.y <= pos_y && self.y + BUTTON_SIZE > pos_y {
                return true;
            }
        }
        return false;
    }
    
}