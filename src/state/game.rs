use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    command::CommandProcessor,
    engine::{graphics::Graphics, music::stop_music, App, Plugins},
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    random::GamePseudoRandom,
    saves::{GameWorldState, Player, SaveManager},
    world_wrapper::WorldRequest,
};

use firecore_battle_engine::{
    battle::default_engine::DefaultBattleEngine,
    pokengine::texture::{ItemTextures, PokemonTextures, TrainerGroupTextures},
    BattleGuiTextures,
};
use worldcli::{
    engine::graphics::Font,
    map::data::ClientWorldData,
    pokedex::trainer::InitTrainer,
    worldlib::map::{battle::BattleId, manager::WorldMapManager},
};

use crate::battle_wrapper::BattleWrapper;
use crate::world_wrapper::WorldWrapper;

pub enum GameStates {
    World,
    Battle,
}

impl Default for GameStates {
    fn default() -> Self {
        Self::World
    }
}

pub(super) struct GameStateManager {
    pub state: GameStates,

    pub world: WorldWrapper,
    pub battle: BattleWrapper,

    pub saves: Rc<RefCell<SaveManager<String>>>,
}

impl GameStateManager {
    pub fn seed(&mut self, seed: u64) {
        self.world.seed(seed);
        self.battle.seed(seed);
    }
}

impl GameStateManager {
    pub fn load(
        gfx: &mut Graphics,
        saves: Rc<RefCell<SaveManager<String>>>,
        pokedex: Arc<Dex<Pokemon>>,
        movedex: Arc<Dex<Move>>,
        itemdex: Arc<Dex<Item>>,
        pokemon: Arc<PokemonTextures>,
        items: Arc<ItemTextures>,
        trainers: Arc<TrainerGroupTextures>,
        debug_font: Font,
        commands: CommandProcessor,
    ) -> Result<Self, String> {
        let btl = postcard::from_bytes::<BattleGuiTextures<Vec<u8>>>(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/battle_gui.bin"
        )))
        .map_err(|err| err.to_string())?
        .init(gfx)?;

        Ok(Self {
            state: Default::default(),
            saves,
            world: WorldWrapper::new(
                WorldMapManager {
                    data: firecore_storage::from_bytes(include_bytes!(concat!(
                        env!("OUT_DIR"),
                        "/world.bin"
                    )))
                    .map_err(|err| err.to_string())?,
                    scripting: postcard::from_bytes(include_bytes!(concat!(
                        env!("OUT_DIR"),
                        "/world_script.bin"
                    )))
                    .map_err(|err| err.to_string())?,
                    pokedex: pokedex.clone(),
                    movedex: movedex.clone(),
                    itemdex: itemdex.clone(),
                },
                ClientWorldData::new(
                    gfx,
                    firecore_storage::from_bytes(include_bytes!(concat!(
                        env!("OUT_DIR"),
                        "/world_textures.bin"
                    )))
                    .map_err(|err| err.to_string())?,
                    debug_font,
                )?,
                pokemon.clone(),
                items.clone(),
                commands.clone(),
            ),
            battle: BattleWrapper::new(
                gfx,
                pokedex,
                movedex,
                itemdex,
                pokemon,
                items,
                trainers,
                btl,
                {
                    let mut engine = DefaultBattleEngine::new::<BattleId, GamePseudoRandom>();

                    engine.moves = postcard::from_bytes(include_bytes!(concat!(
                        env!("OUT_DIR"),
                        "/battle_moves.bin"
                    )))
                    .map_err(|err| err.to_string())?;

                    engine.scripting.moves = postcard::from_bytes(include_bytes!(concat!(
                        env!("OUT_DIR"),
                        "/battle_move_scripts.bin"
                    )))
                    .map_err(|err| err.to_string())?;

                    engine
                },
                commands,
            )?,
        })
    }

    pub fn start(&mut self) {
        match self.state {
            GameStates::World => {
                if let Some(save) = self.saves.borrow_mut().current_mut() {
                    self.world.start(&mut save.world, &mut save.trainer);
                }
            }
            GameStates::Battle => (),
        }
    }

    pub fn end(&self, app: &mut App, plugins: &mut Plugins) {
        stop_music(app, plugins);
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins, delta: f32) {
        if let Some(player) = self.saves.borrow_mut().current_mut() {
            let Player {
                version,
                world,
                trainer,
            } = player;
            let state = world;
            match self.state {
                GameStates::World => {
                    self.world.update(app, plugins, state, trainer, delta);
                    if self.battle.try_battle(&mut state.map.player, trainer) {
                        self.state = GameStates::Battle;
                    }
                }
                GameStates::Battle => {
                    if self.battle.update(app, plugins, &mut state.map.player) {
                        if let Some(winner) = self.battle.winner() {
                            let winner = winner == &BattleId::Player;
                            self.world
                                .manager
                                .post_battle(&mut state.map, trainer, winner);
                        }
                        self.state = GameStates::World;
                        self.world.start(state, trainer);
                    }
                }
            }
        }
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        match self.state {
            GameStates::World => {
                if let Some(player) = self.saves.borrow().current() {
                    self.world.draw(gfx, &player.world.map);
                }
            }
            GameStates::Battle => {
                if self.battle.world_active() {
                    if let Some(player) = self.saves.borrow().current() {
                        self.world.draw(gfx, &player.world.map);
                    }
                }
                self.battle.draw(gfx);
            }
        }
    }

    pub fn ui(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        egui: &crate::engine::egui::Context,
    ) -> bool {
        match self.state {
            GameStates::World => {
                if let Some(player) = self.saves.borrow_mut().current_mut() {
                    if let Some(request) = self.world.ui(
                        app,
                        plugins,
                        egui,
                        &mut player.world.map,
                        &mut player.trainer,
                    ) {
                        match request {
                            WorldRequest::Save => {
                                self.saves.borrow_mut().save();
                                false
                            }
                            WorldRequest::Exit => {
                                return true;
                            }
                        }
                    } else {
                        false
                    }
                } else {
                    true
                }
            }
            GameStates::Battle => self.battle.ui(app, plugins, egui),
        }
    }
}
