extern crate firecore_game as game;
extern crate firecore_world as world;
extern crate firecore_battle as battle;

use game::{
    macroquad::{
        Window,
        camera::{
            Camera2D,
            set_camera,
        },
        prelude::{
            Conf,
            clear_background,
            BLACK,
            get_frame_time,
            next_frame,
            info,
            coroutines::{
                start_coroutine,
                stop_coroutine,
                wait_seconds,
            },
            is_key_down,
            is_key_pressed,
            KeyCode,
            warn,
            draw_rectangle,
            Rect,
            Texture2D,
            BLUE,
        }
    },
    storage::{
        get_mut, 
        configuration::Configuration,
    },
    util::{
        WIDTH,
        HEIGHT,
    },
    text::TextColor,
    graphics::{
        draw,
        draw_text_left,
    },
};

use state::{
    loading::{LOADING_FINISHED, load_coroutine},
    manager::StateManager,
};

use std::sync::atomic::Ordering::Relaxed;

pub mod state;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SCALE: f32 = 3.0;

static mut STATE_MANAGER: Option<StateManager> = None;

#[cfg(target_arch = "wasm32")]
#[game::macroquad::main(settings)]
async fn main() {
    start().await;
}

fn settings() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (WIDTH * DEFAULT_SCALE) as _,
        window_height: (HEIGHT * DEFAULT_SCALE) as _,
        sample_count: 1,
        ..Default::default()
    }
}


#[cfg(not(target_arch = "wasm32"))]
fn main() {

    Window::from_config(settings(), start());

    info!("Quitting game...");

    unsafe { STATE_MANAGER.as_mut().unwrap().quit() };

}

pub async fn start() {

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    set_camera(&game_camera());
    

    // Loads configuration and player saves, sets up controls

    game::init::data().await;

    // Loads fonts

    match postcard::from_bytes(include_bytes!("../build/data/fonts.bin")) {
        Ok(font_sheets) => game::init::text(font_sheets),
        Err(err) => {
            warn!("Could not load font sheets with error {}", err);
            warn!("Game will start with no text display.");
        }
    }

    // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

    let texture = game::graphics::byte_texture(include_bytes!("../build/assets/loading.png"));
    
    // Flash the loading screen once so the screen freezes on this instead of a blank one

    loading_screen(texture);

    let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        start_coroutine(load_coroutine())
    } else {
        start_coroutine(async move {
            loop {
                loading_screen(texture);
                next_frame().await;
            }
        })
    };

    info!("Loading assets...");

    // Parses arguments

    let args = getopts();

    #[cfg(feature = "audio")]
    if !args.contains(&Args::DisableAudio) {

        // Load audio files and setup audio
        match postcard::from_bytes(include_bytes!("../build/data/audio.bin")) {
            Ok(sound) => game::init::audio(sound),
            Err(err) => game::macroquad::prelude::error!("Could not read sound file with error {}", err)
        }
        

    }

    {

        if args.contains(&Args::Debug) {
            game::DEBUG.store(true, Relaxed);
        }
        
        if game::is_debug() {
            info!("Running in debug mode");
        }    

    }

    // Load pokedex and movedex;

    match postcard::from_bytes(include_bytes!("../build/data/dex.bin")) {
        Ok(dex) => game::init::pokedex(dex),
        Err(err) => panic!("Could not load pokedex with error {}", err),
    };

    // Load scenes
   
    unsafe { STATE_MANAGER = Some(StateManager::new()) };

    let state_manager = unsafe { STATE_MANAGER.as_mut().unwrap() };

    state_manager.load().await;

    info!("Finished loading assets!");
    LOADING_FINISHED.store(true, Relaxed);

    // Wait for the loading scenes to finish, then stop the coroutine

    #[cfg(not(target_arch = "wasm32"))] {
        while !loading_coroutine.is_done() {
            wait_seconds(0.1).await;
        } 
    }

    stop_coroutine(loading_coroutine); 

    // Start the loading scenes on wasm32 because they lag in a coroutine

    #[cfg(target_arch = "wasm32")] {
        load_coroutine().await;
    }

    info!("Starting game!");

    state_manager.on_start();

    let mut paused = false;

    loop {

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::P) {
            paused = !paused;
        }

        if !paused {

            #[cfg(all(target_arch = "wasm32", feature = "audio"))]
            firecore_audio::backend::quadsnd::music::MIXER.lock().frame();

            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {

                // Toggle debug on key press

                if is_key_pressed(KeyCode::O) {
                    let debug = !game::DEBUG.load(Relaxed);
                    game::DEBUG.store(debug, Relaxed);
                    info!("Debug: {}", debug)
                }
        
                // Reload configuration on key press
        
                if is_key_pressed(KeyCode::P) {
                    if let Some(mut config) = get_mut::<Configuration>() {
                        if let Err(err) = game::storage::reload(std::ops::DerefMut::deref_mut(&mut config)).await {
                            warn!("Could not reload configuration with error {}", err);
                        }
                    }
                }

            }
            
            state_manager.input(get_frame_time());
            
            state_manager.update(get_frame_time());

        }
    
        clear_background(BLACK);

        state_manager.render();

        // Render touchscreen controls if they are active

        if let Some(touchscreen) = unsafe { game::input::touchscreen::TOUCHSCREEN.as_ref() } {
            draw_touch_button(&touchscreen.a);
            draw_touch_button(&touchscreen.b);
            draw_touch_button(&touchscreen.up);
            draw_touch_button(&touchscreen.down);
            draw_touch_button(&touchscreen.left);
            draw_touch_button(&touchscreen.right);
        }
    
        // Quit game if asked to

        if game::should_quit() {
            draw_rectangle(0.0, 0.0, WIDTH, HEIGHT, BLACK);
            break;
        }

        next_frame().await;
    }

    state_manager.quit();

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
    draw(texture, 0.0, 0.0);
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
	game::graphics::draw_text_left(0, &format!("{:?}", button.control), TextColor::White, button.pos.x + 1.0, button.pos.y);
}

