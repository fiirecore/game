use std::ops::Deref;

use crate::{
    engine::{
        graphics::{Font, Graphics},
        music::stop_music,
        App, Plugins,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    pokengine::PokedexClientData,
    random::GameWorldRandoms,
    saves::GameWorldState,
};

use battlecli::{
    battle::default_engine::{default_scripting::MoveScripts, EngineMoves},
    BattleGuiData,
};

use worldcli::{
    pokedex::trainer::InitTrainer,
    worldlib::{
        map::{battle::BattleId, manager::WorldMapData},
        random::WorldRandoms,
        script::default::DefaultWorldScriptEngine,
        serialized::SerializedTextures,
    },
};

use event::EventWriter;

use crate::command::CommandProcessor;

use crate::battle_wrapper::BattleManager;
use crate::world_wrapper::WorldWrapper;

use super::StateMessage;

pub enum GameStates {
    World,
    Battle,
}

impl Default for GameStates {
    fn default() -> Self {
        Self::World
    }
}

pub(super) struct GameStateManager<
    D: Deref<Target = PokedexClientData> + Clone,
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    state: GameStates,

    world: WorldWrapper<D>,
    battle: BattleManager<D, P, M, I>,

    world_randoms: GameWorldRandoms,
}

impl<
        D: Deref<Target = PokedexClientData> + Clone,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > GameStateManager<D, P, M, I>
{
    pub fn new(
        gfx: &mut Graphics,
        debug_font: Font,
        dex: D,
        btl: BattleGuiData,
        wrld: WorldMapData,
        world_script: DefaultWorldScriptEngine,
        world_tex: SerializedTextures,
        battle: (EngineMoves, MoveScripts),
        sender: EventWriter<StateMessage>,
        commands: CommandProcessor,
    ) -> Result<Self, String> {
        Ok(Self {
            state: GameStates::default(),
            world: WorldWrapper::new(
                gfx,
                dex.clone(),
                sender,
                commands.clone(),
                wrld,
                world_script,
                world_tex,
                debug_font,
            )?,
            battle: BattleManager::new(gfx, btl, dex, battle, commands)?,
            world_randoms: WorldRandoms::default(),
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.world_randoms.seed(seed);
        self.battle.seed(seed);
    }
}

impl<
        D: Deref<Target = PokedexClientData> + Clone,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > GameStateManager<D, P, M, I>
{
    pub fn start(&mut self, state: &mut GameWorldState, trainer: &mut InitTrainer<P, M, I>) {
        match self.state {
            GameStates::World => {
                self.world
                    .manager
                    .start(&mut state.map, &mut self.world_randoms, trainer)
            }
            GameStates::Battle => (),
        }
    }

    pub fn end(&self, app: &mut App, plugins: &mut Plugins) {
        stop_music(app, plugins);
    }

    pub fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        state: &mut GameWorldState,
        trainer: &mut InitTrainer<P, M, I>,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
    ) {
        // Speed game up if spacebar is held down

        let delta = app.timer.delta_f32()
            * if app
                .keyboard
                .is_down(worldcli::engine::notan::prelude::KeyCode::Space)
            {
                4.0
            } else {
                1.0
            };

        match self.state {
            GameStates::World => {
                self.world.update(
                    app,
                    plugins,
                    state,
                    &mut self.world_randoms,
                    trainer,
                    pokedex,
                    movedex,
                    itemdex,
                    delta,
                );
                if self
                    .battle
                    .battle(pokedex, movedex, itemdex, &mut state.map.player, trainer)
                {
                    self.state = GameStates::Battle;
                }
            }
            GameStates::Battle => {
                self.battle.update(
                    app,
                    plugins,
                    &mut state.map.player,
                    pokedex,
                    movedex,
                    itemdex,
                );
                if self.battle.finished {
                    if let Some(winner) = self.battle.winner() {
                        let winner = winner == &BattleId::Player;
                        self.world
                            .manager
                            .post_battle(&mut state.map, trainer, winner);
                    }
                    self.state = GameStates::World;
                    self.world
                        .manager
                        .start(&mut state.map, &mut self.world_randoms, trainer);
                }
            }
        }
        // Ok(())
    }

    pub fn draw(
        &self,
        gfx: &mut Graphics,
        state: &GameWorldState,
    ) {
        match self.state {
            GameStates::World => self.world.draw(gfx, &state.map),
            GameStates::Battle => {
                if self.battle.world_active() {
                    self.world.draw(gfx, &state.map);
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
        state: &mut GameWorldState,
        trainer: &mut InitTrainer<P, M, I>,
    ) {
        match self.state {
            GameStates::World => self.world.ui(app, plugins, egui, &mut state.map, trainer),
            GameStates::Battle => self.battle.ui(app, plugins, egui),
        }
    }
}
