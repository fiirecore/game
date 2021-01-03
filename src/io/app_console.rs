use log::info;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use std::time::SystemTime;
use crate::engine::text::{TEXT_HEIGHT1, TextRenderer};

use crate::entity::entity::Entity;
pub struct AppConsole {

    alive: bool,
    text: Vec<String>,
    background_color: [f32; 4],
    last_message_time: SystemTime,

}

static LINES_TO_RENDER: u8 = 5;
static SHOW_MESSAGE_TIMER: u8 = 5;

impl AppConsole {

    pub fn new() -> AppConsole {
        AppConsole {
            text: Vec::new(),
            background_color: [0.0, 0.0, 0.0, 0.4],
            alive: false,
            last_message_time: SystemTime::now(),
        }
    }

    pub fn add_line(&mut self, string: String) {
        info!("{}", &string);
        self.text.push(string);
        self.last_message_time = SystemTime::now();
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
        if self.alive && self.last_message_time.elapsed().unwrap().as_secs() < SHOW_MESSAGE_TIMER as u64 {
            crate::util::render_util::draw_rect(ctx, g, self.background_color.into(), 5, 5, 128, 104);
            let mut min = self.text.len() as isize - LINES_TO_RENDER as isize;
            if min < 0 {
                min = 0;
            }
            let mut count = 0;
            for index in min as usize..self.text.len() {
                //crate::util::render_util::draw_text(c, gl, gc, self.text[index].as_str(), 10, 20 + count * 20);
                tr.render_text_from_left(ctx, g, 1, self.text[index].as_str(), 10, 10 + count * (TEXT_HEIGHT1 + 2) as isize);
                count+=1;
            }
        }
    }

    pub fn toggle(&mut self) {
        self.alive = !self.alive;
    }

}

impl Entity for AppConsole {

    fn spawn(&mut self) {
        self.alive = true;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        return self.alive;
    }

}