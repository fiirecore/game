use game::{
    tetra::{
        State, Context, Result, Event,
        graphics::{
            self, Color,
            scaling::{ScreenScaler, ScalingMode},
        },
    },
    log::{info, error},
};

use crate::args::Args;

use super::{
	MainState,
	MainStates,
    menu::MenuStateManager,
    game::GameStateManager,
};

pub struct StateManager {

    current: MainStates,

    menu: MenuStateManager,
    game: GameStateManager,

    args: Vec<Args>,
    scaler: ScreenScaler,

}

impl State for StateManager {
    fn begin(&mut self, ctx: &mut Context) -> Result {
        // Loads fonts
    
        match game::deps::ser::deserialize(include_bytes!("../../build/data/fonts.bin")) {
            Ok(font_sheets) => game::init::text(ctx, font_sheets)?,
            Err(err) => {
                error!("Could not load font sheets with error {}", err);
                error!("Game will start with no text display.");
            }
        }

        // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)
    
        // let texture = game::graphics::byte_texture(include_bytes!("../build/assets/loading.png"));
        
        // Flash the loading screen once so the screen freezes on this instead of a blank one
    
        // loading_screen(texture);
    
        // let loading_coroutine = if cfg!(not(target_arch = "wasm32")) {
        //     start_coroutine(load_coroutine())
        // } else {
        //     start_coroutine(async move {
        //         loop {
        //             loading_screen(texture);
        //             next_frame().await;
        //         }
        //     })
        // };
    
        info!("Loading assets...");
    
        // Parses arguments
    
        // let args = getopts();
    
        #[cfg(feature = "audio")]
        if !self.args.contains(&Args::DisableAudio) {
            //Load audio files and setup audio
            match game::deps::ser::deserialize(include_bytes!("../../build/data/audio.bin")) {
                Ok(sound) => game::init::audio(sound),
                Err(err) => error!("Could not read sound file with error {}", err)
            }
        }
    
        {
    
            if self.args.contains(&Args::Debug) {
                game::set_debug(true);
            }
            
            if game::is_debug() {
                info!("Running in debug mode");
            }    
    
        }

        // Load pokedex and movedex;

        match game::deps::ser::deserialize(include_bytes!("../../build/data/dex.bin")) {
            Ok(dex) => game::init::pokedex(ctx, dex)?,
            Err(err) => panic!("Could not deserialize pokedex with error {}", err),
        }

        // loads player saves
        
        game::storage::init();
    
        #[cfg(debug_assertions)] {
			let saves = unsafe{game::storage::PLAYER_SAVES.as_mut()}.expect("Could not get player saves");
			if saves.saves.is_empty() {
				self.current = MainStates::Menu;
			} else {
				saves.select(0);
			}			
		}

        self.game.load(ctx);

        info!("Finished loading!");
		self.get_mut().begin(ctx)
    }
    fn end(&mut self, ctx: &mut Context) -> Result {
        self.get_mut().end(ctx)
    }
    fn update(&mut self, ctx: &mut Context) -> Result {
        self.get_mut().update(ctx)?;
        if let Some(state) = self.get_mut().next().take() {
			self.get_mut().end(ctx)?;
			self.current = state;
			self.get_mut().begin(ctx)?;
		}
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);
        self.get_mut().draw(ctx)?;
        graphics::reset_transform_matrix(ctx);
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);
        Ok(())
    }
    fn event(&mut self, _: &mut Context, event: Event) -> Result {
        if let Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}
impl StateManager {

    pub fn new(ctx: &mut Context, args: Vec<Args>) -> Result<Self> {
        // graphics::set_canvas(ctx, canvas)
        Ok(Self {
            current: MainStates::default(),
            menu: MenuStateManager::new(ctx),
            game: GameStateManager::new(ctx),
            args,
            scaler: ScreenScaler::with_window_size(ctx, game::util::WIDTH as _, game::util::HEIGHT as _, ScalingMode::ShowAll)?,
        })
    }

    // pub async fn load(&mut self) {
	// 	self.game.load().await;
	// }

    // fn get(&self) -> &dyn MainState {
	// 	match self.current {
	// 	    MainStates::Menu => &self.menu,
	// 	    MainStates::Game => &self.game,
	// 	}
	// }

	fn get_mut(&mut self) -> &mut dyn MainState {
		match self.current {
		    MainStates::Menu => &mut self.menu,
		    MainStates::Game => &mut self.game,
		}
	}
}