use game::{
    util::{Entity, Reset, Completable, WIDTH, HEIGHT},
    macroquad::{
        camera::{set_camera, Camera2D},
        prelude::{Color, Rect, draw_rectangle},
    },
};

use crate::transitions::{BattleTransition, BattleScreenTransition};

const DEFAULT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.0);
const FINAL_INDEX: u8 = 4;

pub struct FlashBattleScreenTransition {
    active: bool,
    finished: bool,
    screen: Color,
    waning: bool,
    index: u8,
    fade: f32,
    zoom: bool,
    zoom_offset: f32,
}

impl FlashBattleScreenTransition {
    pub fn new() -> Self {
        Self {
            active: false,
            finished: false,

            screen: DEFAULT_COLOR,
            waning: false,
            index: 0,
            fade: 1.0 / 8.0,
            zoom: false,
            zoom_offset: 0.0,
        }
    }
}

impl BattleScreenTransition for FlashBattleScreenTransition {}

impl BattleTransition for FlashBattleScreenTransition {

    fn on_start(&mut self) {
        
    }

    fn update(&mut self, delta: f32) {
        if self.waning {
            self.screen.a -= self.fade * 60.0 * delta;
        } else {
            self.screen.a += self.fade * 60.0 * delta;
        }
        if self.screen.a <= 0.0 {
            self.waning = false;
            self.index += 1;
        } else if self.screen.a >= 1.0 {
            self.waning = true;
        }
        if self.index == FINAL_INDEX && self.screen.a <= 0.0 {
            self.screen.r = 0.0;
            self.screen.g = 0.0;
            self.screen.b = 0.0;
            self.fade = 1.0 / 16.0;
            self.zoom = true;
        }
        if self.zoom {
            self.zoom_offset += 600.0 * delta;
            set_camera(Camera2D::from_display_rect(Rect::new(self.zoom_offset / 2.0, self.zoom_offset / 2.0, WIDTH - self.zoom_offset, HEIGHT - self.zoom_offset)))
        }
        if self.index >= FINAL_INDEX && self.waning {
            set_camera(Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT)));
            self.finished = true;
        }
    }

    fn render(&self) {
        draw_rectangle(
            0.0,
            0.0,
            WIDTH,
            HEIGHT,
            self.screen,
        );
    }

}

impl Reset for FlashBattleScreenTransition {
    fn reset(&mut self) {
        self.screen = DEFAULT_COLOR;
        self.waning = false;
        self.index = 0;
        self.fade = 1.0 / 8.0;
        self.zoom = false;
        self.zoom_offset = 0.0;
    }
}

impl Completable for FlashBattleScreenTransition {
    fn is_finished(&self) -> bool {
        return self.finished;
    }
}

impl Entity for FlashBattleScreenTransition {
    fn spawn(&mut self) {
        self.active = true;
        self.finished = false;
    }

    fn despawn(&mut self) {
        self.active = false;
        self.finished = false;
    }

    fn is_alive(&self) -> bool {
        self.active
    }
}
