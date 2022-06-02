use std::ops::Deref;

use crate::{
    engine::{
        graphics::{Font, Graphics},
        music::stop_music,
        App, Plugins,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    pokengine::PokedexClientData,
};

use battlecli::{
    battle::default_engine::{scripting::MoveScripts, EngineMoves},
    BattleGuiData,
};

use worldcli::worldlib::{
    character::player::InitPlayerCharacter, map::battle::BattleId, serialized::SerializedWorld,
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
        wrld: SerializedWorld,
        battle: (EngineMoves, MoveScripts),
        sender: EventWriter<StateMessage>,
        commands: CommandProcessor,
    ) -> Result<Self, String> {
        Ok(Self {
            state: GameStates::default(),
            world: WorldWrapper::new(gfx, dex.clone(), sender, commands.clone(), wrld, debug_font)?,
            battle: BattleManager::new(gfx, btl, dex, battle, commands)?,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.world.seed(seed);
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
    pub fn start(&mut self, player: &mut InitPlayerCharacter<P, M, I>) {
        match self.state {
            GameStates::World => self.world.manager.start(player),
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
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
        player: &mut InitPlayerCharacter<P, M, I>,
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
                self.world
                    .update(app, plugins, player, pokedex, movedex, itemdex, delta);
                if self.battle.battle(pokedex, movedex, itemdex, player) {
                    self.state = GameStates::Battle;
                }
            }
            GameStates::Battle => {
                self.battle
                    .update(app, plugins, player, pokedex, movedex, itemdex);
                if self.battle.finished {
                    if let Some(winner) = self.battle.winner() {
                        let winner = winner == &BattleId::Player;
                        self.world.manager.post_battle(player, winner);
                    }
                    self.state = GameStates::World;
                    self.world.manager.start(player);
                }
            }
        }
        // Ok(())
    }

    pub fn draw(&self, gfx: &mut Graphics, player: &InitPlayerCharacter<P, M, I>) {
        match self.state {
            GameStates::World => self.world.draw(gfx, player),
            GameStates::Battle => {
                if self.battle.world_active() {
                    self.world.draw(gfx, player);
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
        player: &mut InitPlayerCharacter<P, M, I>,
    ) {
        match self.state {
            GameStates::World => self.world.ui(app, plugins, egui, player),
            GameStates::Battle => self.battle.ui(app, plugins, egui),
        }
    }
}
