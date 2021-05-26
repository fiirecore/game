pub extern crate firecore_game as game;

use game::{
    macroquad::{
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
    loading::load_coroutine,
    manager::StateManager,
};

pub mod state;


pub static mut STATE_MANAGER: Option<StateManager> = None;

pub fn settings() -> Conf {
    Conf {
        window_title: TITLE.to_string(),
        window_width: (WIDTH * DEFAULT_SCALE) as _,
        window_height: (HEIGHT * DEFAULT_SCALE) as _,
        sample_count: 1,
        ..Default::default()
    }
}


pub async fn start() {

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

    // Load scenes

    state_manager.load().await;

    info!("Finished loading assets!");

    #[cfg(not(feature = "audio"))]
    game::init::finished_loading();
    #[cfg(feature = "audio")]
    if game::is_debug() {
        game::init::finished_loading()
    }

    #[cfg(debug_assertions)]
    if args.contains(&Args::Seed) {
        game::init::seed_randoms(game::macroquad::miniquad::date::now() as u64);
    }

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

            // #[cfg(all(target_arch = "wasm32", feature = "audio"))]
            // game::firecore::backend::quadsnd::music::MIXER.lock().frame();

            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {

                // Toggle debug on key press

                if is_key_pressed(KeyCode::O) {
                    let debug = !game::is_debug();
                    game::set_debug(debug);
                    info!("Debug Mode: {}", debug)
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
            
            state_manager.update(get_frame_time().min(0.5));

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

pub fn loading_screen(texture: Texture2D) {
    clear_background(BLUE);
    draw(texture, 0.0, 0.0);
    draw_text_left(0,crate::VERSION, TextColor::White, 1.0, 1.0);
    draw_text_left(1, "The game may stay on this screen", TextColor::White, 5.0, 50.0);
    draw_text_left(1, "for up to two minutes.", TextColor::White, 5.0, 65.0);
}

pub const CAMERA_SIZE: Rect = Rect { x: 0.0, y: 0.0, w: game::util::WIDTH, h: game::util::HEIGHT };

pub fn draw_touch_button(button: &game::input::touchscreen::TouchButton) {
	game::macroquad::prelude::draw_rectangle(button.pos.x, button.pos.y, game::input::touchscreen::TouchButton::BUTTON_SIZE, game::input::touchscreen::TouchButton::BUTTON_SIZE, button.color);
	game::graphics::draw_text_left(0, &format!("{:?}", button.control), TextColor::White, button.pos.x + 1.0, button.pos.y);
}

