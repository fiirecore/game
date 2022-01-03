use crate::{
    command::CommandProcessor,
    engine::{
        error::EngineError,
        graphics::{self, Color},
        log::{error, info},
        Context, EngineContext, State,
    },
    saves::Player,
    state::MainStates,
    load::LoadContext,
};

use firecore_world::events::{split, Receiver, Sender};

use super::{
    console::Console, game::GameStateManager, loading::LoadingStateManager, menu::MenuStateManager,
    StateMessage,
};

pub struct StateManager {
    state: MainStates,

    loading: LoadingStateManager,
    menu: MenuStateManager,
    game: GameStateManager,

    console: Console,

    save: Option<Player>,

    sender: Sender<StateMessage>,
    receiver: Receiver<StateMessage>,
}

impl State<EngineContext> for StateManager {
    fn start(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        info!("Starting game!");
        match self.state {
            MainStates::Loading => (), //self.loading.start()
            MainStates::Menu => self.menu.start(ctx, eng),
            MainStates::Game => self.game.start(ctx),
        }
    }
    fn end(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        match self.state {
            MainStates::Loading => (),
            MainStates::Menu => self.menu.end(ctx, eng),
            MainStates::Game => self.game.end(),
        }
        self.process(ctx, eng);
    }
    fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32) {
        if let Some(command) = self.console.update(
            ctx,
            delta,
            self.game.save.as_mut().map(|p| p.player.as_mut()).flatten(),
        ) {
            match self.state {
                MainStates::Loading => (),
                MainStates::Menu => (),
                MainStates::Game => self.game.process(command),
            }
        }

        match self.state {
            MainStates::Loading => self.loading.update(ctx, eng, delta),
            MainStates::Menu => self.menu.update(ctx, eng, delta, &mut self.save),
            MainStates::Game => self.game.update(ctx, eng, delta, self.console.alive()),
        }

        self.process(ctx, eng);

        // Ok(())
    }
    fn draw(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        // graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);
        match self.state {
            MainStates::Loading => self.loading.draw(ctx, eng),
            MainStates::Menu => self.menu.draw(ctx, eng),
            MainStates::Game => self.game.draw(ctx, eng),
        }
        self.console.draw(ctx, eng);
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
    pub(crate) fn new(ctx: &mut Context, _: &mut EngineContext, load: LoadContext) -> Self {
        Self::try_new(ctx, load)
            .unwrap_or_else(|err| panic!("Could not create state manager with error {}", err))
    }

    fn try_new(ctx: &mut Context, load: LoadContext) -> Result<Self, EngineError> {
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

        use crate::engine::graphics::{set_scaling_mode, ScalingMode};

        set_scaling_mode(ctx, ScalingMode::Stretch, Some(crate::SCALE));

        let (sender, receiver) = split();

        Ok(Self {
            state: MainStates::default(),

            loading: LoadingStateManager::new(ctx, sender.clone())?,
            menu: MenuStateManager::new(ctx, sender.clone())?,
            game: GameStateManager::new(
                ctx,
                load.dex,
                load.btl,
                load.world,
                load.battle,
                sender.clone(),
            )?,

            console: Console::default(),

            save: load.save,

            sender,
            receiver,
            // scaler,
        })
    }

    pub fn process(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        for message in self.receiver.try_iter() {
            match message {
                StateMessage::UpdateSave(save) => {
                    self.save = Some(save);
                    self.sender.send(StateMessage::WriteSave);
                }
                StateMessage::WriteSave => {
                    if let Some(save) = &self.save {
                        if let Err(err) = storage::save::<storage::RonSerializer, Player>(
                            save,
                            crate::PUBLISHER,
                            crate::APPLICATION,
                        ) {
                            error!("Cannot write save with error {}", err);
                        }
                    }
                }
                StateMessage::LoadSave => {
                    if let Some(save) = self.save.take() {
                        self.game.save = Some(save);
                    }
                }
                StateMessage::Goto(state) => {
                    match self.state {
                        MainStates::Loading => (),
                        MainStates::Menu => self.menu.end(ctx, eng),
                        MainStates::Game => self.game.end(),
                    }
                    self.state = state;
                    match self.state {
                        MainStates::Loading => self.loading.start(ctx, eng),
                        MainStates::Menu => self.menu.start(ctx, eng),
                        MainStates::Game => self.game.start(ctx),
                    }
                }
                StateMessage::Seed(seed) => self.game.seed(seed as _),
                StateMessage::Exit => ctx.quit(),
                StateMessage::CommandError(error) => self.console.error(error),
            }
        }
    }
}
