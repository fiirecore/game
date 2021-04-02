use firecore_util::{Direction, text::TextColor};

use firecore_input::Control;
use macroquad::camera::Camera2D;
use macroquad::prelude::{warn, Rect, Texture2D, clear_background, BLUE, WHITE, screen_width, screen_height};

use crate::util::graphics::draw_text_left;

pub mod graphics;
pub mod text;
pub mod pokemon;

pub const TILE_SIZE: u8 = 16;

pub fn play_music(id: firecore_audio::MusicId) {
    if let Err(err) = firecore_audio::play_music_id(id) {
        warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(music: &str) {
    if let Err(err) = firecore_audio::play_music_named(music) {
        warn!("Could not play music \"{}\" with error {}", music, err);
    }
}

#[cfg(feature = "audio")]
pub async fn load_audio() {

    use macroquad::prelude::error;
    
    if let Err(err) = firecore_audio::create() {
        error!("Could not create audio instance with error {}", err);
    } else {
        
        let audio = bincode::deserialize(
            // &macroquad::prelude::load_file("assets/audio.bin").await.unwrap()
            include_bytes!("../../build/data/audio.bin")
        ).unwrap();

        #[cfg(not(target = "wasm32"))] {
            std::thread::spawn( || {
                if let Err(err) = firecore_audio::load(audio) {
                    error!("Could not load audio files with error {}", err);
                }
            });
        }
        #[cfg(target = "wasm32")] {
            if let Err(err) = firecore_audio::load(audio) {
                error!("Could not load audio files with error {}", err);
            }
        }

    }

}

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
                    macroquad::prelude::warn!("Could not parse command line arguments with error {}", f.to_string());
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
    macroquad::prelude::draw_texture(texture, 0.0, 0.0, WHITE);
    draw_text_left(0,crate::VERSION, TextColor::White, 1.0, 1.0);
    draw_text_left(1, "The game may stay on this screen", TextColor::White, 5.0, 50.0);
    draw_text_left(1, "for up to two minutes.", TextColor::White, 5.0, 65.0);
}

pub fn keybind(direction: Direction) -> Control {
	match direction {
		Direction::Up => Control::Up,
		Direction::Down => Control::Down,
		Direction::Left => Control::Left,
		Direction::Right => Control::Right,
	}
}

pub const CAMERA_SIZE: Rect = Rect { x: 0.0, y: 0.0, w: crate::WIDTH, h: crate::HEIGHT };

pub fn window_camera() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()))
}

pub fn game_camera() -> Camera2D {
    Camera2D::from_display_rect(CAMERA_SIZE)
}