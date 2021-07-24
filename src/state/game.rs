use std::rc::Rc;

use crate::{
    deps::ser,
    engine::{
        tetra::{
            input::{is_key_down, Key},
            time::get_delta_time,
            Context, Result, State,
        },
        util::Entity,
    },
    game::{
        battle_glue::BattleEntry,
        storage::{
            data_mut,
            player::PlayerSave,
            save_locally,
        },
    },
    pokedex::gui::{bag::BagGui, party::PartyGui},
};

use command::Console;

use log::warn;

use crate::{
    battle::BattleManager,
    state::{MainState, MainStates},
    world::map::manager::WorldManager,
};

pub mod command;

pub struct GameStateManager {
    state: Option<MainStates>,

    gamestate: GameStates,

    world: WorldManager,
    battle: BattleManager,

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

impl GameStateManager {
    pub fn new(ctx: &mut Context) -> Self {
        let party = Rc::new(PartyGui::new(ctx));
        let bag = Rc::new(BagGui::new(ctx));

        Self {
            state: None,

            gamestate: GameStates::default(),

            world: WorldManager::new(ctx, party.clone(), bag.clone()),
            battle: BattleManager::new(ctx, party, bag),

            battle_entry: None,

            console: Console::default(),
        }
    }

    pub fn load(&mut self, ctx: &mut Context) {
        match ser::deserialize(include_bytes!("../../build/data/world.bin")) {
            Ok(world) => self.world.load(ctx, world),
            Err(err) => panic!("Could not load world file with error {}", err),
        }
    }

    pub fn data_dirty(&mut self, save: &mut PlayerSave) {
        self.save_data(save);
        save.should_save = false;
    }

    pub fn save_data(&mut self, save: &mut PlayerSave) {
        self.world.save_data(save);
        if let Err(err) = save.save(save_locally()) {
            warn!(
                "Could not save player data for {} with error {}",
                save.name, err
            );
        }
    }
}

impl State for GameStateManager {
    fn begin(&mut self, ctx: &mut Context) -> Result {
        self.world.load_with_data();
        self.world.on_start(ctx, &mut self.battle_entry);
        Ok(())
    }

    fn end(&mut self, _: &mut Context) -> Result {
        self.save_data(data_mut());
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result {
        if let Some(command) = self.console.update(ctx) {
            match self.gamestate {
                GameStates::World => self.world.process(command, &mut self.battle_entry),
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

        let save = data_mut();
        if save.should_save {
            self.data_dirty(save);
        }
        match self.gamestate {
            GameStates::World => {
                self.world.update(
                    ctx,
                    delta,
                    self.console.alive(),
                    &mut self.battle_entry,
                    &mut self.state,
                );
                if let Some(entry) = self.battle_entry.take() {
                    if self.battle.battle(entry) {
                        self.gamestate = GameStates::Battle;
                    }
                }
            }
            GameStates::Battle => {
                self.battle.update(ctx, delta);
                if self.battle.finished {
                    let p = &mut self.world.world.player;
                    p.input_frozen = false;
                    p.unfreeze();
                    let save = data_mut();
                    if let Some(winner) = self.battle.winner() {
                        let trainer = self.battle.update_data(&winner, save);
                        self.world.update_world(save, winner, trainer);
                    }
                    self.gamestate = GameStates::World;
                    self.world.map_start(ctx, true);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        match self.gamestate {
            GameStates::World => self.world.draw(ctx),
            GameStates::Battle => {
                if self.battle.world_active() {
                    self.world.draw(ctx);
                }
                self.battle.draw(ctx);
            }
        }
        self.console.draw(ctx);
        Ok(())
    }
}

impl MainState for GameStateManager {
    fn next(&mut self) -> Option<MainStates> {
        self.state.take()
    }
}
