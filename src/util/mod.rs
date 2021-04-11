use game::graphics::draw_text_left;
use game::util::text::TextColor;

use game::macroquad::{
    camera::Camera2D,
    prelude::{
        warn,
        Texture2D, clear_background, BLUE, WHITE,
        Rect,
        draw_texture,
        // screen_width, screen_height
    }
};

pub const TILE_SIZE: f32 = 16.0;

#[derive(PartialEq)]
pub enum Args {

    DisableAudio,
    Debug,

}

pub fn getopts() -> Vec<Args> {

    #[cfg(not(target_arch = "wasm32"))] {
        let mut list = Vec::new();
        let args: Vec<String> = std::env::args().collect();
        let mut opts = getopts::Options::new();

        opts.optflag("a", "disable-audio", "Disable audio");
        opts.optflag("d", "debug", "Add debug keybinds and other stuff");

        if args.len() > 0 {
            match opts.parse(&args[1..]) {
                Ok(m) => {
                    if m.opt_present("a") {
                        list.push(Args::DisableAudio);
                    }
                    if m.opt_present("d") {
                        list.push(Args::Debug);
                    }
                }
                Err(f) => {
                    warn!("Could not parse command line arguments with error {}", f.to_string());
                }
            };
        }

        list
    }
    #[cfg(target_arch = "wasm32")] {
        Vec::new()
    }
}

pub fn loading_screen(texture: Texture2D) {
    clear_background(BLUE);
    draw_texture(texture, 0.0, 0.0, WHITE);
    draw_text_left(0,crate::VERSION, TextColor::White, 1.0, 1.0);
    draw_text_left(1, "The game may stay on this screen", TextColor::White, 5.0, 50.0);
    draw_text_left(1, "for up to two minutes.", TextColor::White, 5.0, 65.0);
}

pub fn seed_randoms(seed: u64) {
    game::init::seed_randoms(seed);
    world::seed_randoms(seed);
	battle::BATTLE_RANDOM.seed(seed);
}

pub const CAMERA_SIZE: Rect = Rect { x: 0.0, y: 0.0, w: game::util::WIDTH, h: game::util::HEIGHT };

// pub fn window_camera() -> Camera2D {
//     Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()))
// }

pub fn game_camera() -> Camera2D {
    Camera2D::from_display_rect(CAMERA_SIZE)
}

pub fn draw_touch_button(button: &game::input::touchscreen::TouchButton) {
	game::macroquad::prelude::draw_rectangle(button.pos.x, button.pos.y, game::input::touchscreen::TouchButton::BUTTON_SIZE, game::input::touchscreen::TouchButton::BUTTON_SIZE, button.color);
	game::graphics::draw_text_left(0, &format!("{:?}", button.control), game::util::text::TextColor::White, button.pos.x + 1.0, button.pos.y);
}

