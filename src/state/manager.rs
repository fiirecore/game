use firecore_battle::pokedex::Uninitializable;
use rand::prelude::SmallRng;

use crate::{
    command::CommandProcessor,
    engine::{
        graphics::{self, Color},
        log::info,
        Context, State,
    },
    saves::PlayerSaves,
    state::MainStates,
    LoadContext,
};

use firecore_world::events::{split, Receiver};

use super::{console::Console, game::GameStateManager, menu::MenuStateManager, StateMessage};

pub struct StateManager {
    state: MainStates,

    menu: MenuStateManager,
    game: GameStateManager,

    console: Console,

    saves: PlayerSaves,

    random: SmallRng,

    receiver: Receiver<StateMessage>,
}

impl State for StateManager {
    fn start(&mut self, ctx: &mut Context) {
        info!("Starting game!");
        match self.state {
            MainStates::Menu => self.menu.start(ctx, &self.saves.list),
            MainStates::Game => self.game.start(ctx),
        }
    }
    fn end(&mut self, ctx: &mut Context) {
        match self.state {
            MainStates::Menu => self.menu.end(ctx),
            MainStates::Game => self.game.end(),
        }
    }
    fn update(&mut self, ctx: &mut Context, delta: f32) {

        if let Some(command) = self.console.update(ctx, delta, self.game.save.as_mut().map(|p| &mut p.character)) {
            match self.state {
                MainStates::Menu => (),
                MainStates::Game => self.game.process(command),
            }
        }

        match self.state {
            MainStates::Menu => self.menu.update(ctx, delta, &mut self.saves.list),
            MainStates::Game => self.game.update(ctx, delta, self.console.alive()),
        }

        for message in self.receiver.try_iter() {
            match message {
                StateMessage::UseSave(save) => self.game.save = save.init(&mut self.random),
                StateMessage::UpdateSave(save) => {
                    let save = save.uninit();
                    if let Some(pos) = self.saves.list.iter().position(|s| s.id == save.id) {
                        self.saves.list[pos] = save;
                    } else {
                        self.saves.list.push(save);
                    }
                }
                StateMessage::Save => {}
                StateMessage::Goto(state) => {
                    match self.state {
                        MainStates::Menu => self.menu.end(ctx),
                        MainStates::Game => self.game.end(),
                    }
                    self.state = state;
                    match self.state {
                        MainStates::Menu => self.menu.start(ctx, &self.saves.list),
                        MainStates::Game => self.game.start(ctx),
                    }
                }
                StateMessage::Seed(seed) => self.game.seed(seed as _),
                StateMessage::Exit => ctx.quit(),
                StateMessage::CommandError(error) => self.console.error(error),
            }
        }

        // Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) {
        // graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);
        match self.state {
            MainStates::Menu => self.menu.draw(ctx, &self.saves.list),
            MainStates::Game => self.game.draw(ctx),
        }
        self.console.draw(ctx);
        // graphics::reset_transform_matrix(ctx);
        // graphics::reset_canvas(ctx);
        // graphics::clear(ctx, Color::BLACK);
        // self.scaler.draw(ctx);
        // Ok(())
    }
    // fn event(&mut self, _: &mut GameContext, event: Event) {
    //     if let Event::Resized { width, height } = event {
    //         self.scaler.set_outer_size(width, height);
    //     }

    //     Ok(())
    // }
}
impl StateManager {
    pub(crate) fn new(ctx: &mut Context, load: LoadContext) -> Self {
        // Creates a quick loading screen and then starts the loading scene coroutine (or continues loading screen on wasm32)

        // let texture = game::graphics::Texture::new(include_bytes!("../build/assets/loading.png"));

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

        use crate::engine::graphics::{ScalingMode, set_scaling_mode};

        set_scaling_mode(ctx, ScalingMode::Stretch, Some(crate::SCALE));

        let (sender, receiver) = split();

        Self {
            state: MainStates::default(),

            menu: MenuStateManager::new(ctx, sender.clone())
                .unwrap_or_else(|err| panic!("Could not create main menu with error {}", err)),
            game: GameStateManager::new(ctx, load.dex, load.btl, load.world, load.battle, sender).unwrap_or_else(|err| panic!("Could not create game state with error {}", err)),

            console: Console::default(),

            saves: load.saves,
            random: load.random,

            receiver,
            // scaler,
        }
    }
}
