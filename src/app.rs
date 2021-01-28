
//use piston_window::{Button, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, EventLoop};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use macroquad::prelude::info;
use enum_iterator::IntoEnumIterator;

use crate::BASE_HEIGHT;
use crate::BASE_WIDTH;
use crate::TITLE;
use crate::scene::scene_manager::SceneManager;
use crate::util::Quit;
use crate::util::input::Control;
use crate::util::text_renderer::TextRenderer;

use crate::util::context::GameContext;
use crate::util::Load;

pub struct Game {

    scene_manager: SceneManager,
    game_context: GameContext,
    text_renderer: TextRenderer,

}

impl Game {

    pub fn new() -> Game {

        ;

        Game {
            scene_manager: SceneManager::new(),
            game_context: GameContext::new(),
            text_renderer: TextRenderer::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {

        //let scale = crate::CONFIGURATION.lock().unwrap().window_scale as u32;
        

        let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new(
            TITLE,
            [
                BASE_WIDTH * scale,
                BASE_HEIGHT * scale,
            ],
        )
        .resizable(true)
        .vsync(true)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });

        window.set_ups(60);

        let mut g = GlGraphics::new(opengl_graphics::OpenGL::V3_2);

        let controls = Control::into_enum_iter();

        self.load()?;
        self.on_start();

        while let Some(e) = window.next() {

            if let Some(_) = e.update_args() {
                self.scene_manager.input(&mut self.game_context);
                for control in controls {
                    self.game_context.key_update(control);
                }
                self.fkey_update();
                self.scene_manager.update(&mut self.game_context);
            }

            if let Some(args) = e.render_args() {
                let size = args.window_size[1] / BASE_HEIGHT as f64;
                g.draw(args.viewport(), | ref mut ctx, g| {
                    *ctx = ctx.scale(size, size);
                    piston_window::clear([0.0, 0.0, 0.0, 1.0], g);
                    self.scene_manager.render(&mut self.text_renderer);
                });                
            }

            if let Some(ref button) = e.press_args() {
                if Button::Keyboard(piston_window::Key::R).eq(button) {
                    window.set_size([BASE_WIDTH * scale, BASE_HEIGHT * scale]);
                }
                self.game_context.key_press(button);
                self.key_press(button);
            }

            if let Some(ref button) = e.release_args() {
                self.game_context.key_release(button);
                self.key_release(button);
            }
        }

        self.quit();

        Ok(())
    }
    

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        println!("Loading...");

        // let mut log_name = String::from("logs/log_");
        // log_name.push_str(Instant::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string().as_str());
        // log_name.push_str(".txt");

        // if !PathBuf::from("logs").exists() {
        //     std::fs::create_dir("logs").expect("Could not create logs directory!");
        // }
        
        // CombinedLogger::init(
        //     vec![
        //         TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        //         WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log_name).unwrap()),
        //     ]
        // )?;

        self.text_renderer.load_textures();

        self.text_renderer.default_add();

        self.scene_manager.load();

        self.game_context.fill_keymaps();

        info!("Done loading!");

        Ok(())
    }

    pub fn on_start(&mut self) {
        self.scene_manager.on_start(&mut self.game_context);
    }

    pub fn key_press(&mut self, button: &Button) {
        if let Some(i) = self.game_context.fkeymap.get(button) {
            if i < &self.game_context.fkeys.len() {
                self.game_context.fkeys[*i] = 1;
            }
        }
    }

    pub fn key_release(&mut self, button: &Button) {
        if let Some(i) = self.game_context.fkeymap.get(button) {
            if i < &self.game_context.fkeys.len() {
                self.game_context.fkeys[*i] = 3;
            }
        }
    }

    pub fn fkey_update(&mut self) {

        for i in 0..self.game_context.fkeys.len() {
            if self.game_context.fkeys[i] == 1 {
                self.game_context.fkeys[i] = 2;
            }
        }

        for i in 0..self.game_context.fkeys.len() {
            if self.game_context.fkeys[i] == 3 {
                self.game_context.fkeys[i] = 0;
            }
        }
    }

    pub fn quit(&mut self) {
        info!("Closing app...");
        self.scene_manager.quit();
    }
}