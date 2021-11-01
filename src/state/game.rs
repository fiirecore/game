use pokedex::Uninitializable;
use saves::PlayerData;
use std::rc::Rc;

use crate::{
    engine::{
        tetra::{
            input::{is_key_down, Key},
            time::get_delta_time,
            Result, State,
        },
        util::Entity,
    },
    game::battle_glue::{BattleEntry, BattleId},
    pokedex::gui::{bag::BagGui, party::PartyGui},
};

use command::Console;

use log::warn;

use crate::{
    battle::BattleManager, state::MainState, world::map::manager::WorldManager, GameContext,
};

use super::Action;

pub mod command;

pub struct GameStateManager<'d> {
    state: Option<Action>,

    gamestate: GameStates,

    world: WorldManager,
    battle: BattleManager<'d>,

    battle_entry: Option<BattleEntry>,

    console: Console,
}

pub enum GameStates {
    World,
    Battle,
}

impl Default for GameStates {
    fn default() -> Self {
        Self::World
    }
}

impl<'d> GameStateManager<'d> {
    pub fn new(ctx: &mut GameContext<'d>) -> Self {
        let party = Rc::new(PartyGui::new(&ctx.dex));
        let bag = Rc::new(BagGui::new(&ctx.dex));

        Self {
            state: None,

            gamestate: GameStates::default(),

            world: WorldManager::new(ctx, party.clone(), bag.clone()),
            battle: BattleManager::new(&mut ctx.engine, &ctx.dex, party, bag),

            battle_entry: None,

            console: Console::default(),
        }
    }

    pub fn load(&mut self, ctx: &mut GameContext) {
        match bincode::deserialize(include_bytes!("../../build/data/world.bin")) {
            Ok(world) => self.world.load(ctx, world),
            Err(err) => panic!("Could not load world file with error {}", err),
        }
    }

    pub fn data_dirty(&mut self, save: &mut PlayerData, local: bool) {
        self.save_data(save, local);
        save.should_save = false;
    }

    pub fn save_data(&mut self, save: &mut PlayerData, local: bool) {
        self.world.save_data(save);
        if let Err(err) = save.clone().uninit().save(local) {
            warn!(
                "Could not save player data for {} with error {}",
                save.name, err
            );
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.world.randoms.seed(seed);
    }
}

impl<'d> State<GameContext<'d>> for GameStateManager<'d> {
    fn begin(&mut self, ctx: &mut GameContext<'d>) -> Result {
        let save = ctx.saves.get_mut();
        self.world.load_with_data(save);
        self.world
            .on_start(&mut ctx.engine, save, &mut self.battle_entry);
        Ok(())
    }

    fn end(&mut self, ctx: &mut GameContext<'d>) -> Result {
        self.save_data(ctx.saves.get_mut(), ctx.save_locally);
        Ok(())
    }

    fn update(&mut self, ctx: &mut GameContext<'d>) -> Result {
        if let Some(command) = self.console.update(&mut ctx.engine) {
            match self.gamestate {
                GameStates::World => self.world.process(
                    command,
                    &ctx.dex,
                    ctx.saves.get_mut(),
                    &mut self.battle_entry,
                ),
                GameStates::Battle => warn!("Battle has no commands implemented."),
            }
        }

        // Speed game up if spacebar is held down

        let delta = get_delta_time(ctx).as_secs_f32()
            * if is_key_down(ctx, Key::Space) {
                4.0
            } else {
                1.0
            };

        let save = ctx.saves.get_mut();
        if save.should_save {
            self.data_dirty(save, ctx.save_locally);
        }
        match self.gamestate {
            GameStates::World => {
                self.world.update(
                    &mut ctx.engine,
                    &ctx.dex,
                    save,
                    delta,
                    self.console.alive(),
                    &mut self.battle_entry,
                    &mut self.state,
                );
                if let Some(entry) = self.battle_entry.take() {
                    if self.battle.battle(save, &ctx.dex, entry) {
                        self.gamestate = GameStates::Battle;
                    }
                }
            }
            GameStates::Battle => {
                self.battle.update(&mut ctx.engine, &ctx.dex, delta, save);
                if self.battle.finished {
                    let p = &mut self.world.world.player;
                    p.input_frozen = false;
                    p.unfreeze();
                    if let Some(winner) = self.battle.winner() {
                        let winner = *winner;
                        let save = ctx.saves.get_mut();
                        let trainer = self.battle.update_data(&winner, save);
                        if let BattleId(Some(winner)) = winner {
                            self.world.update_world(save, winner, trainer);
                        }
                    }
                    self.gamestate = GameStates::World;
                    self.world.map_start(&mut ctx.engine, true);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut GameContext<'d>) -> Result {
        match self.gamestate {
            GameStates::World => self.world.draw(&mut ctx.engine, &ctx.dex, ctx.saves.get()),
            GameStates::Battle => {
                if self.battle.world_active() {
                    self.world.draw(&mut ctx.engine, &ctx.dex, ctx.saves.get());
                }
                self.battle.draw(&mut ctx.engine, &ctx.dex, ctx.saves.get());
            }
        }
        self.console.draw(&mut ctx.engine);
        Ok(())
    }
}

impl<'d> MainState<'d> for GameStateManager<'d> {
    fn action(&mut self) -> Option<Action> {
        self.state.take()
    }
}
