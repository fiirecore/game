// use firecore_util::text::TextColor;
use macroquad::prelude::Color;
use macroquad::prelude::Touch;
use macroquad::prelude::TouchPhase;
use macroquad::prelude::Vec2;

use crate::Control;

pub static mut TOUCHSCREEN: Option<TouchControls> = None;

pub fn touchscreen(active: bool) {
    unsafe { 
        TOUCHSCREEN = if active {
            Some(TouchControls::default())
        } else {
            None
        } 
    }
}

pub struct TouchControls {

    pub a: TouchButton,
    pub b: TouchButton,

    pub down: TouchButton,
    pub up: TouchButton,
    pub left: TouchButton,
    pub right: TouchButton,

}

impl TouchControls {

    // pub fn render(&self) {
    //     self.a.draw();
    //     self.b.draw();
    //     self.down.draw();
    //     self.up.draw();
    //     self.left.draw();
    //     self.right.draw();
    // }

    pub fn pressed(&self, control: &Control) -> bool {
        // if let Some(config) = get::<Configuration>() {
        //     if config.touchscreen {
        let touches = macroquad::input::touches();
        if !touches.is_empty() {
            if self.a.touching(&touches, true) {
                if self.a.control.eq(control) {
                    return true;
                }
            }
            if self.b.touching(&touches, true) {
                if self.b.control.eq(control) {
                    return true;
                }
            }
            if self.up.touching(&touches, true) {
                if self.up.control.eq(control) {
                    return true;
                }
            }
            if self.down.touching(&touches, true) {
                if self.down.control.eq(control) {
                    return true;
                }
            }
            if self.left.touching(&touches, true) {
                if self.left.control.eq(control) {
                    return true;
                }
            }
            if self.right.touching(&touches, true) {
                if self.right.control.eq(control) {
                    return true;
                }
            }
        }
            // }
        // }
        return false;
    }

    pub fn down(&self, control: &Control) -> bool {
        // if let Some(config) = get::<Configuration>() {
        //     if config.touchscreen {
        let touches = macroquad::input::touches();
        if !touches.is_empty() {
            if self.a.touching(&touches, false) {
                if self.a.control.eq(control) {
                    return true;
                }
            }
            if self.b.touching(&touches, false) {
                if self.b.control.eq(control) {
                    return true;
                }
            }
            if self.up.touching(&touches, false) {
                if self.up.control.eq(control) {
                    return true;
                }
            }
            if self.down.touching(&touches, false) {
                if self.down.control.eq(control) {
                    return true;
                }
            }
            if self.left.touching(&touches, false) {
                if self.left.control.eq(control) {
                    return true;
                }
            }
            if self.right.touching(&touches, false) {
                if self.right.control.eq(control) {
                    return true;
                }
            }
        }
            // }
        // }  
        false
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

    pub pos: Vec2,
    pub color: Color, // replace with texture
    pub control: Control,

}

impl TouchButton {

    pub const BUTTON_SIZE: f32 = 20.0;

    pub fn new(x: f32, y: f32, control: Control) -> Self {
        Self {
            pos: Vec2::new(x, y),
            color: macroquad::prelude::RED,
            control,
        }
    }

    // pub fn draw(&self) {
    //     // if let Some(config) = get::<Configuration>() {
    //     //     if config.touchscreen {
    //             crate::util::graphics::draw_rect(self.color, self.pos.x, self.pos.y, BUTTON_SIZE, BUTTON_SIZE);
    //             crate::util::graphics::draw_text_left(1, &format!("{:?}", self.control), TextColor::White, self.pos.x + 1.0, self.pos.y);
    //     //     }
    //     // }
    // }

    pub fn touching(&self, touches: &Vec<Touch>, pressed: bool) -> bool {
        for touch in touches {
            if !pressed {
                if touch.phase != TouchPhase::Started {
                    break;
                }
            }
            if self.pos.x <= touch.position.x && self.pos.x + Self::BUTTON_SIZE > touch.position.x {
                if self.pos.y <= touch.position.y && self.pos.y + Self::BUTTON_SIZE > touch.position.y {
                    return true;
                }
            }
        }
        false
    }
    
}