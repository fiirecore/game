use std::{cell::RefCell, rc::Rc, sync::Arc};

use firecore_battle_client::pokengine::{
    texture::{ItemTextures, PokemonTextures, TrainerGroupTextures},
    TrainerGroupId,
};
use worldcli::{
    engine::{graphics::CreateFont, notan::prelude::KeyCode, HashMap, AppState, music::MusicId},
    pokedex::Dex,
};

use crate::{
    engine::{
        graphics::{Color, CreateDraw, Graphics},
        App, Plugins,
    },
    saves::SaveManager,
    state::MainStates, settings::Settings,
};

use super::{
    console::Console, game::GameStateManager, loading::LoadingStateManager,
    menu::title::TitleState, StateManager,
};

use crate::engine::notan;

#[derive(AppState)]
pub struct Game {
    state: StateManager<MainStates>,

    loading: LoadingStateManager,

    title: TitleState,
    game: GameStateManager,

    saves: Rc<RefCell<SaveManager<String>>>,
    
    settings: Rc<Settings>,

    console: Console,
}

impl Game {
    pub fn new(app: &mut App, plugins: &mut Plugins, gfx: &mut Graphics) -> Self {
        Self::try_new(app, plugins, gfx)
            .unwrap_or_else(|err| panic!("Could not create state manager with error {}", err))
    }

    pub fn try_new(
        app: &mut App,
        plugins: &mut Plugins,
        gfx: &mut Graphics,
    ) -> Result<Self, String> {
        let (console, processor) = Console::new();

        let pokedex: Arc<Dex<_>> = Arc::new(
            postcard::from_bytes(include_bytes!(concat!(env!("OUT_DIR"), "/pokedex.bin")))
                .map_err(|err| format!("Could not create pokedex with error {}", err))?,
        );
        let movedex: Arc<Dex<_>> = Arc::new(
            postcard::from_bytes(include_bytes!(concat!(env!("OUT_DIR"), "/movedex.bin")))
                .map_err(|err| format!("Could not create movedex with error {}", err))?,
        );

        let itemdex: Arc<Dex<_>> = Arc::new(
            postcard::from_bytes(include_bytes!(concat!(env!("OUT_DIR"), "/itemdex.bin")))
                .map_err(|err| format!("Could not create itemdex with error {}", err))?,
        );

        let (pokemon, cries) = PokemonTextures::new(
            gfx,
            postcard::from_bytes(include_bytes!(concat!(
                env!("OUT_DIR"),
                "/pokemon_textures.bin"
            )))
            .map_err(|err| err.to_string())?,
        )
        .map_err(|err| format!("Could not create pokemon textures with error {}", err))?;

        crate::pokengine::add_cries(app, plugins, cries);

        let pokemon = Arc::new(pokemon);

        let items = Arc::new(
            ItemTextures::new(
                gfx,
                postcard::from_bytes(include_bytes!(concat!(
                    env!("OUT_DIR"),
                    "/item_textures.bin"
                )))
                .map_err(|err| err.to_string())?,
            )
            .map_err(|err| format!("Could not create item textures with error {}", err))?,
        );

        let mut trainer_groups = TrainerGroupTextures::default();

        let trainer_groups_bytes: HashMap<TrainerGroupId, Vec<u8>> = postcard::from_bytes(
            include_bytes!(concat!(env!("OUT_DIR"), "/trainer_textures.bin")),
        )
        .map_err(|err| err.to_string())?;

        for (id, texture) in trainer_groups_bytes {
            trainer_groups.insert(
                id,
                gfx.create_texture()
                    .from_image(&texture)
                    .build()
                    .map_err(|err| {
                        format!("Could not create trainer texture with error {}", err)
                    })?,
            );
        }

        let trainer_groups = Arc::new(trainer_groups);

        let saves = Rc::new(RefCell::new(SaveManager::new(
            pokedex.clone(),
            movedex.clone(),
            itemdex.clone(),
        )));

        for (id, music) in
            postcard::from_bytes::<HashMap<MusicId, Vec<u8>>>(include_bytes!(concat!(env!("OUT_DIR"), "/music.bin")))
                .map_err(|err| format!("Cannot decode music binary with error {err}"))?
        {
            crate::engine::music::add_music(app, plugins, id, music);
        }

        let debug_font = gfx.create_font(include_bytes!("../../assets/ThaleahFat.ttf"))?;

        let settings = Rc::new(Settings::default());

        Ok(Self {
            state: Default::default(),

            loading: LoadingStateManager::new(gfx)?,

            title: TitleState::new(
                gfx,
                firecore_storage::from_bytes(include_bytes!(concat!(
                    env!("OUT_DIR"),
                    "/title.bin"
                )))
                .map_err(|err| err.to_string())?,
            )?,
            game: GameStateManager::load(
                gfx,
                saves.clone(),
                settings.clone(),
                pokedex,
                movedex,
                itemdex,
                pokemon,
                items,
                trainer_groups,
                debug_font,
                processor.clone(),
            )?,

            saves,

            console,

            settings,
            // scaler,
        })
    }

    pub fn start(&mut self, state: MainStates, app: &mut App, plugins: &mut Plugins) {
        match state {
            MainStates::Loading => (),
            MainStates::Title => {
                self.title.start(app, plugins);
            }
            MainStates::Menu => (),
            MainStates::Game => {
                self.game.start();
            }
            MainStates::Error(..) => (),
        }
    }
    pub fn end(&mut self, state: MainStates, app: &mut App, plugins: &mut Plugins) {
        match state {
            MainStates::Loading => (),
            MainStates::Title => {
                self.title.end(app, plugins);
            }
            MainStates::Menu => (),
            MainStates::Game => {
                self.game.end(app, plugins);
            }
            MainStates::Error(..) => (),
        }
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins) {
        let delta = app.timer.delta_f32()
            * if app.keyboard.is_down(KeyCode::Space) {
                4.0
            } else {
                1.0
            };

        match self.state.current {
            MainStates::Loading => {
                if self.loading.update(app, plugins, delta, true) {
                    self.state.queue(MainStates::Title);
                }
            }
            MainStates::Title => {
                // if let Some(title) = self.title.as_mut() {
                if let Some(seed) = self.title.update(app, plugins) {
                    self.state.queue(MainStates::Menu);
                    self.game.seed(seed as _);
                }
                // } else {
                //     self.state.queue(MainStates::Error(
                //         "Could not open title screen as it is not loaded!",
                //     ));
                // }
            }
            MainStates::Menu => (),
            MainStates::Game => {
                // if let Some(game) = self.game.as_mut() {
                self.game.update(app, plugins, delta);
                // }
            }
            MainStates::Error(..) => (),
        }

        if let Some(next) = self.state.next.take() {
            self.end(self.state.current, app, plugins);
            self.start(next, app, plugins);
            self.state.update(next);
        }
    }

    pub fn draw(&mut self, app: &mut App, plugins: &mut Plugins, gfx: &mut Graphics) {
        match self.state.current {
            MainStates::Loading => self.loading.draw(gfx, None),
            MainStates::Title => {
                self.title.draw(gfx);
            }
            MainStates::Menu => {
                let mut draw = gfx.create_draw();
                draw.clear(Color::new(0.00, 0.32, 0.67, 1.0));
                gfx.render(&draw);
            }
            MainStates::Game => {
                self.game.draw(gfx);
            }
            MainStates::Error(..) => (),
        }

        // match &self.state {
        //     MainStates::Loading | MainStates::Title | MainStates::Menu => {
        //         if let LoadData::Load(assets, ..) = &self.data {
        //             let mut draw = gfx.create_draw();

        //             draw.rect(
        //                 (draw.width() / 5.0, draw.height() - draw.height() / 5.0),
        //                 (
        //                     draw.width() * 3.0 / 5.0 * assets.progress(),
        //                     draw.height() / 15.0,
        //                 ),
        //             )
        //             .color(Color::WHITE);

        //             gfx.render(&draw);
        //         }
        //     }
        //     _ => (),
        // }

        use crate::engine::notan::egui::{self, EguiPluginSugar};
        let plugins2: &mut Plugins = unsafe { &mut *(plugins as *mut _) };
        gfx.render(&plugins2.egui(|egui| {
            match self.state.current {
                MainStates::Menu => {

                    self.settings.ui(app, plugins, egui);

                    egui::Window::new("Menu").show(egui, |ui| {
                        if ui.button("Play").clicked() {
                            if self.saves.borrow().current.is_some() {
                                self.state.queue(MainStates::Game);
                            }
                        }
                        if ui.button("New Save / Reset Save").clicked() {
                            let mut saves = self.saves.borrow_mut();
                            let id = format!(
                                "{:?}",
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                            );
                            saves.create(id.clone(), "Red", "Blue");
                            saves.set_current(id);
                        }
                        if ui.button("Settings").clicked() {
                            self.settings.spawn();
                        }
                        if ui.button("Exit").clicked() {
                            let state = self.state.current;
                            self.end(state, app, plugins);
                            app.exit();
                        }
                    });
                }
                MainStates::Game => {
                    if self.game.ui(app, plugins, egui) {
                        self.state.queue(MainStates::Menu);
                    }
                }
                MainStates::Error(error) => {
                    egui::Window::new("Error").show(egui, |ui| {
                        ui.label(error);
                        if ui.button("Exit to Loading Screen").clicked() {
                            self.state.queue(MainStates::Loading);
                        }
                    });
                }
                _ => (),
            }

            #[cfg(target_arch = "wasm32")]
            crate::touchscreen::Touchscreen::ui(egui, plugins2);

            self.console.ui(
                app,
                egui,
                self.saves
                    .borrow_mut()
                    .current_mut()
                    .map(|s| &mut s.world.map.player.character),
            );
        }));

        // graphics::reset_transform_matrix(ctx);
        // graphics::reset_canvas(ctx);
        // graphics::clear(ctx, Color::BLACK);
        // self.scaler.draw(ctx);
        // Ok(())
    }

    // #[deprecated]
    // pub fn process(&mut self, app: &mut App, plugins: &mut Plugins) {
    //     while let Some(message) = self.receiver.read() {
    //         match message {
    //             StateMessage::ResetSave => {
    //                 self.save = save::PlayerSave::Uninit(Player::new("Red", "Blue"));
    //             }
    //             StateMessage::SaveToDisk => {
    //                 if let Some(save) = self.save.cloned_uninit() {
    //                     use firecore_storage as storage;

    //                     impl storage::PersistantData for Player {
    //                         fn path() -> &'static str {
    //                             "save"
    //                         }
    //                     }

    //                     if let Err(err) = storage::save::<storage::RonSerializer, Player>(
    //                         &save,
    //                         Some(crate::PUBLISHER),
    //                         crate::APPLICATION,
    //                     ) {
    //                         crate::engine::log::error!("Cannot write save with error {}", err);
    //                     }
    //                 }
    //             }
    //             StateMessage::Seed(seed) => match &mut self.game {
    //                 GameStateInit::Uninit(.., s) => *s = seed,
    //                 GameStateInit::Init(game) => game.seed(seed as _),
    //                 GameStateInit::None => (),
    //             },
    //             StateMessage::Goto(state) => {
    //                 self.end(app, plugins);
    //                 self.state = state;
    //                 self.start(app, plugins);
    //             }
    //             StateMessage::Exit => app.exit(),
    //         }
    //     }
    // }
}
